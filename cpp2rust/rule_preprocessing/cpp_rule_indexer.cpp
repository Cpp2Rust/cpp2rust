// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/Basic/TokenKinds.h>
#include <clang/Frontend/ASTUnit.h>
#include <clang/Tooling/Tooling.h>
#include <clang/Tooling/Transformer/SourceCode.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/ADT/SmallVector.h>
#include <llvm/ADT/StringExtras.h>
#include <llvm/ADT/StringRef.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/raw_ostream.h>

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <format>
#include <fstream>
#include <memory>
#include <string>
#include <vector>

#include "compat/platform_flags.h"
#include "converter/rules/rules_loader.h"

namespace fs = std::filesystem;

namespace cpp2rust {

struct RuleCtx {
  fs::path path;
  std::string name;
  fs::path index_dir;
  std::vector<std::string> common_headers;
};

std::string ClassOf(clang::QualType type) {
  while (type->isPointerType() || type->isReferenceType()) {
    type = type->getPointeeType();
  }
  if (auto spec = type->getAs<clang::TemplateSpecializationType>()) {
    auto decl = spec->getTemplateName().getAsTemplateDecl();
    if (decl && llvm::isa<clang::ClassTemplateDecl>(decl)) {
      return RulesLoader::ClassKey(decl);
    }
    return {};
  }
  if (auto name = type->getAs<clang::DependentNameType>()) {
    auto qualifier = name->getQualifier();
    if (qualifier.getKind() == clang::NestedNameSpecifier::Kind::Type) {
      return ClassOf(clang::QualType(qualifier.getAsType(), 0));
    }
  }
  return {};
}

std::string ClassOfParameters(const clang::Decl *rule) {
  auto fn = rule->getAsFunction();
  if (!fn) {
    return {};
  }
  for (const auto *param : fn->parameters()) {
    if (auto class_key = ClassOf(param->getType()); !class_key.empty()) {
      return class_key;
    }
  }
  return {};
}

const clang::Expr *RuleExpr(const clang::Decl *decl) {
  if (auto var = llvm::dyn_cast<clang::VarDecl>(decl)) {
    return var->getInit();
  }
  auto fn = decl->getAsFunction();
  if (!fn) {
    return nullptr;
  }
  auto body = llvm::dyn_cast_or_null<clang::CompoundStmt>(fn->getBody());
  if (!body || body->size() != 1) {
    return nullptr;
  }
  auto ret = llvm::dyn_cast<clang::ReturnStmt>(body->body_front());
  return ret ? ret->getRetValue() : nullptr;
}

const clang::Expr *IgnoreImplicitAndFunctionalCast(const clang::Expr *expr) {
  expr = expr->IgnoreImplicit();
  if (auto cast = llvm::dyn_cast<clang::CXXFunctionalCastExpr>(expr)) {
    expr = cast->getSubExpr()->IgnoreImplicit();
  }
  return expr;
}

std::string DependentExprKey(const clang::Expr *expr) {
  auto member = llvm::dyn_cast<clang::CXXDependentScopeMemberExpr>(expr);
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (auto callee = call->getDirectCallee()) {
      return RulesLoader::FunctionKey(callee);
    }
    auto callee = call->getCallee()->IgnoreParenImpCasts();
    member = llvm::dyn_cast<clang::CXXDependentScopeMemberExpr>(callee);
    if (auto overloads = llvm::dyn_cast<clang::UnresolvedMemberExpr>(callee);
        overloads && overloads->decls_begin() != overloads->decls_end()) {
      return RulesLoader::DeclKey(
          (*overloads->decls_begin())->getUnderlyingDecl());
    }
    if (auto lookup = llvm::dyn_cast<clang::UnresolvedLookupExpr>(callee);
        lookup && lookup->getQualifier() &&
        !llvm::isa<clang::CXXOperatorCallExpr>(call) &&
        lookup->decls_begin() != lookup->decls_end()) {
      return RulesLoader::DeclKey(
          (*lookup->decls_begin())->getUnderlyingDecl());
    }
  }
  if (member && !member->isArrow() &&
      member->getBaseType()
          .getNonReferenceType()
          ->getAs<clang::TemplateSpecializationType>()) {
    return RulesLoader::MemberKey(ClassOf(member->getBaseType()),
                                  RulesLoader::MemberName(member->getMember()));
  }
  if (auto construct =
          llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(expr)) {
    auto class_key = ClassOf(construct->getTypeAsWritten());
    return RulesLoader::MemberKey(class_key,
                                  class_key.substr(class_key.rfind(':') + 1));
  }
  return {};
}

std::string ExprKey(clang::ASTContext &ctx, const clang::Decl *rule) {
  auto expr = RuleExpr(rule);
  if (!expr) {
    return {};
  }
  expr = IgnoreImplicitAndFunctionalCast(expr);
  if (!expr->isTypeDependent() && !expr->isValueDependent()) {
    return RulesLoader::ExprKey(ctx, expr);
  }
  return DependentExprKey(expr);
}

std::string TypeKey(const clang::TypedefNameDecl *rule) {
  auto type = rule->getUnderlyingType();
  while (type->isPointerType() || type->isReferenceType()) {
    type = type->getPointeeType();
  }
  if (!type->isDependentType()) {
    return RulesLoader::TypeKey(type);
  }
  return ClassOf(type);
}

std::string CreateIncFile(const RuleCtx &dir, bool is_c,
                          const std::string &name, const std::string &text) {
  if (is_c) {
    return std::format("#define {0} cpp2rust_rules_{1}_{0}\n{2}#undef {0}\n",
                       name, dir.name, text);
  }
  std::string includes;
  if (!dir.common_headers.empty()) {
    includes = std::format("#ifndef CPP2RUST_RULES_{0}_INCLUDES\n"
                           "#define CPP2RUST_RULES_{0}_INCLUDES\n",
                           dir.name);
    for (const auto &header : dir.common_headers) {
      includes += std::format("#include \"{}\"\n", header);
    }
    includes += "#endif\n";
  }
  return std::format("namespace cpp2rust_rules_{} {{\n{}{}}}\n", dir.name,
                     includes, text);
}

void IndexRuleFile(clang::ASTContext &ctx, const RuleCtx &dir) {
  bool is_c = !ctx.getLangOpts().CPlusPlus;
  auto index_dir = dir.index_dir / (is_c ? "c" : "cpp");
  auto file_name = dir.name + ".inc";
  auto &sm = ctx.getSourceManager();
  for (auto *decl : ctx.getTranslationUnitDecl()->decls()) {
    if (decl->isImplicit() ||
        !sm.isInMainFile(sm.getExpansionLoc(decl->getLocation()))) {
      continue;
    }
    std::string name;
    if (auto named = llvm::dyn_cast<clang::NamedDecl>(decl)) {
      name = named->getQualifiedNameAsString();
    }
    llvm::StringRef number = name;
    if (!(number.consume_front("f") || number.consume_front("t")) ||
        number.empty() || !llvm::all_of(number, llvm::isDigit)) {
      llvm::errs() << "ERROR: declaration '" << name << "' in rule dir "
                   << dir.path.string() << " is not a rule\n";
      std::exit(EXIT_FAILURE);
    }
    auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(decl);
    if (auto tmpl = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(decl)) {
      alias = tmpl->getTemplatedDecl();
    }
    auto key = alias ? TypeKey(alias) : ExprKey(ctx, decl);
    if (key.empty()) {
      key = ClassOfParameters(decl);
    }
    if (key.empty()) {
      llvm::errs() << "ERROR: cannot derive the key of rule '" << name
                   << "' in rule dir " << dir.path.string() << '\n';
      std::exit(EXIT_FAILURE);
    }
    auto text =
        clang::tooling::getExtendedText(*decl, clang::tok::semi, ctx).str() +
        '\n';
    auto path = index_dir / RulesLoader::IndexPath(key) / file_name;
    fs::create_directories(path.parent_path());
    std::ofstream(path, std::ios::app) << CreateIncFile(dir, is_c, name, text);
  }
}

void Index(const fs::path &src_path, const RuleCtx &dir,
           llvm::ArrayRef<llvm::StringRef> cxx_flags) {
  bool is_c = src_path.extension() == ".c";
  auto flags = getPlatformClangBeginFlags();
  flags.push_back("-isystem" + src_path.parent_path().string());
  if (!is_c) {
    flags.insert(flags.end(), cxx_flags.begin(), cxx_flags.end());
  }
  auto end_flags = getPlatformClangEndFlags();
  flags.insert(flags.end(), end_flags.begin(), end_flags.end());
  auto code = llvm::MemoryBuffer::getFile(src_path.string());
  if (!code) {
    llvm::errs() << "ERROR: cannot read " << src_path.string() << '\n';
    std::exit(EXIT_FAILURE);
  }
  auto ast = clang::tooling::buildASTFromCodeWithArgs(
      (*code)->getBuffer(), flags, src_path.string(),
      is_c ? CLANG_C_COMPILER : CLANG_CXX_COMPILER);
  if (!ast || ast->getDiagnostics().hasErrorOccurred()) {
    llvm::errs() << "ERROR: cannot parse " << src_path.string() << '\n';
    std::exit(EXIT_FAILURE);
  }
  IndexRuleFile(ast->getASTContext(), dir);
}

} // namespace cpp2rust

namespace {

llvm::cl::OptionCategory cat("cpp-rule-indexer options");

llvm::cl::opt<std::string>
    SrcDir("dir",
           llvm::cl::desc("Path to a rule directory containing src.c and/or "
                          "src.cpp."),
           llvm::cl::value_desc("rule-dir"), llvm::cl::Required,
           llvm::cl::cat(cat));

llvm::cl::opt<std::string>
    RulePath("path",
             llvm::cl::desc("Path of the rule directory relative to the "
                            "rules root, e.g. std/vector."),
             llvm::cl::value_desc("rule-path"), llvm::cl::Required,
             llvm::cl::cat(cat));

llvm::cl::opt<std::string>
    IndexDir("index", llvm::cl::desc("Directory of the rule index to write."),
             llvm::cl::value_desc("index-dir"), llvm::cl::Required,
             llvm::cl::cat(cat));

llvm::cl::list<std::string>
    CommonHeaders("common-header",
                  llvm::cl::desc("Header included by every indexed C++ rule"),
                  llvm::cl::value_desc("header"), llvm::cl::ZeroOrMore,
                  llvm::cl::cat(cat));

llvm::cl::list<std::string> CXXFlags("cxxflags",
                                     llvm::cl::desc("Additional CXXFLAGS"),
                                     llvm::cl::value_desc("cxxflags"),
                                     llvm::cl::ZeroOrMore, llvm::cl::cat(cat));

} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cat);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  cpp2rust::RuleCtx dir;
  dir.path = RulePath.getValue();
  dir.name = RulePath.getValue();
  std::ranges::replace(dir.name, '/', '_');
  dir.index_dir = IndexDir.getValue();
  dir.common_headers.assign(CommonHeaders.begin(), CommonHeaders.end());

  llvm::SmallVector<llvm::StringRef, 4> cxx_flags(CXXFlags.begin(),
                                                  CXXFlags.end());
  for (const char *name : {"src.c", "src.cpp"}) {
    auto path = fs::path(SrcDir.getValue()) / name;
    if (!fs::exists(path)) {
      continue;
    }
    llvm::errs() << "Indexing " << path.string() << '\n';
    cpp2rust::Index(path, dir, cxx_flags);
  }
  return EXIT_SUCCESS;
}
