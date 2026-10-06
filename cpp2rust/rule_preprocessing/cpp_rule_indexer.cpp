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
#include <llvm/ADT/StringExtras.h>
#include <llvm/ADT/StringRef.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/JSON.h>
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
};

fs::path ClassOf(clang::QualType type) {
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

fs::path ClassOfParameters(const clang::Decl *rule) {
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

fs::path DependentExprKey(const clang::Expr *expr) {
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
    return RulesLoader::MemberKey(class_key, class_key.filename().string());
  }
  return {};
}

fs::path ExprKey(clang::ASTContext &ctx, const clang::Decl *rule) {
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

fs::path TypeKey(const clang::TypedefNameDecl *rule) {
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
  return std::format("namespace cpp2rust_rules_{} {{\n{}}}\n", dir.name, text);
}

void AddRule(llvm::json::Object &rules, std::string key,
             llvm::json::Value rule) {
  auto &slot = rules[llvm::json::ObjectKey(std::move(key))];
  if (!slot.getAsArray()) {
    slot = llvm::json::Array();
  }
  slot.getAsArray()->push_back(std::move(rule));
}

void IndexRuleFile(clang::ASTContext &ctx, const RuleCtx &dir,
                   const fs::path &src_path, llvm::json::Object &rules) {
  bool is_c = !ctx.getLangOpts().CPlusPlus;
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
    auto range = clang::tooling::getExtendedRange(*decl, clang::tok::semi, ctx);
    auto text = std::format(
        "#line {} \"{}\"\n{}\n", sm.getSpellingLineNumber(range.getBegin()),
        src_path.string(), clang::tooling::getText(range, ctx).str());
    llvm::json::Object rule{{"text", CreateIncFile(dir, is_c, name, text)}};
    if (!is_c) {
      rule["namespace"] = "cpp2rust_rules_" + dir.name;
    }
    AddRule(rules, key.generic_string(), std::move(rule));
  }
}

void Index(const fs::path &src_path, const RuleCtx &dir,
           const std::vector<std::string> &cxx_flags,
           llvm::json::Object &rules) {
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
  IndexRuleFile(ast->getASTContext(), dir, src_path, rules);
}

void WriteJson(const fs::path &path, llvm::json::Object object) {
  std::error_code ec;
  llvm::raw_fd_ostream os(path.string(), ec);
  if (ec) {
    llvm::errs() << "ERROR: cannot write " << path.string() << ": "
                 << ec.message() << '\n';
    std::exit(EXIT_FAILURE);
  }
  os << llvm::json::Value(std::move(object));
}

std::vector<std::string> ReadCXXFlags(const fs::path &rule_dir) {
  std::vector<std::string> flags;
  std::ifstream file(rule_dir / "cxxflags");
  for (std::string flag; std::getline(file, flag);) {
    if (!flag.empty()) {
      flags.push_back(flag);
    }
  }
  return flags;
}

void IndexRules(const fs::path &rules_dir, const fs::path &index_dir,
                const std::vector<std::string> &excluded,
                const std::vector<std::string> &common_headers) {
  std::vector<fs::path> rule_dirs;
  for (const auto &entry : fs::directory_iterator(rules_dir)) {
    if (entry.is_directory() &&
        !llvm::is_contained(excluded, entry.path().filename().string())) {
      rule_dirs.push_back(entry.path());
    }
  }
  std::ranges::sort(rule_dirs);
  fs::create_directories(index_dir);
  for (const char *lang : {"c", "cpp"}) {
    std::string common;
    if (lang == std::string("cpp")) {
      for (const auto &header : common_headers) {
        common += std::format("#include \"{}\"\n", header);
      }
    }
    llvm::json::Object rules;
    for (const auto &rule_dir : rule_dirs) {
      auto src_path = rule_dir / (std::string("src.") + lang);
      if (!fs::exists(src_path)) {
        continue;
      }
      llvm::errs() << "Indexing " << src_path.string() << '\n';
      RuleCtx dir;
      dir.path = rule_dir.filename();
      dir.name = rule_dir.filename().string();
      Index(src_path, dir, ReadCXXFlags(rule_dir), rules);
    }
    WriteJson(index_dir / (std::string(lang) + ".json"),
              llvm::json::Object{{"common", std::move(common)},
                                 {"rules", std::move(rules)}});
  }
}

} // namespace cpp2rust

namespace {

llvm::cl::OptionCategory cat("cpp-rule-indexer options");

llvm::cl::opt<std::string>
    RulesDir("rules",
             llvm::cl::desc("Path to the rules directory, whose subdirectories "
                            "contain src.c and/or src.cpp."),
             llvm::cl::value_desc("rules-dir"), llvm::cl::Required,
             llvm::cl::cat(cat));

llvm::cl::opt<std::string>
    IndexDir("index", llvm::cl::desc("Directory of the rule index to write."),
             llvm::cl::value_desc("index-dir"), llvm::cl::Required,
             llvm::cl::cat(cat));

llvm::cl::list<std::string>
    Excluded("exclude", llvm::cl::desc("Rule directory that is not indexed"),
             llvm::cl::value_desc("rule-dir"), llvm::cl::ZeroOrMore,
             llvm::cl::cat(cat));

llvm::cl::list<std::string> CommonHeaders(
    "common-header", llvm::cl::desc("Header the C++ rules depend on"),
    llvm::cl::value_desc("header"), llvm::cl::ZeroOrMore, llvm::cl::cat(cat));

} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cat);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  cpp2rust::IndexRules(RulesDir.getValue(), IndexDir.getValue(),
                       {Excluded.begin(), Excluded.end()},
                       {CommonHeaders.begin(), CommonHeaders.end()});
  return EXIT_SUCCESS;
}
