// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rules_loader.h"

#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Lex/Preprocessor.h>
#include <clang/Sema/Sema.h>
#include <llvm/ADT/DenseSet.h>
#include <llvm/Support/MemoryBuffer.h>

#include <cctype>
#include <filesystem>
#include <format>
#include <set>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/printer.h"
#include "converter/rules/matcher.h"
#include "logging.h"

namespace cpp2rust::RulesLoader {

namespace {

namespace fs = std::filesystem;

class ImplicitMemberDefiner
    : public clang::RecursiveASTVisitor<ImplicitMemberDefiner> {
public:
  explicit ImplicitMemberDefiner(clang::Sema &sema) : sema_(sema) {}

  bool shouldVisitTemplateInstantiations() const { return true; }

  bool VisitCXXRecordDecl(clang::CXXRecordDecl *decl) {
    if (decl->isThisDeclarationADefinition() && !decl->isDependentType() &&
        IsConvertibleCXXRecordDecl(decl) &&
        (decl->isStruct() || decl->isClass()) && !decl->isAbstract()) {
      DefineImplicitMembers(sema_, decl);
    }
    return true;
  }

private:
  clang::Sema &sema_;
};

class KeyCollector : public clang::RecursiveASTVisitor<KeyCollector> {
public:
  KeyCollector(clang::Sema &sema, std::set<std::string> &paths)
      : ctx_(sema.Context), sema_(sema), paths_(paths) {}

  bool shouldVisitTemplateInstantiations() const { return true; }

  bool shouldVisitImplicitCode() const { return true; }

  bool TraverseDecl(clang::Decl *decl) {
    if (decl && !llvm::isa<clang::TranslationUnitDecl>(decl) &&
        ctx_.getSourceManager().isInSystemHeader(decl->getLocation())) {
      return true;
    }
    return RecursiveASTVisitor::TraverseDecl(decl);
  }

  bool VisitExpr(clang::Expr *expr) {
    if (expr->isTypeDependent() || expr->isValueDependent()) {
      return true;
    }
    paths_.insert(
        IndexPath(false, Matcher::ExprKey(Printer::ToString(ctx_, expr))));
    AddType(expr->getType());
    return true;
  }

  bool VisitValueDecl(clang::ValueDecl *decl) {
    AddType(decl->getType());
    return true;
  }

  bool VisitDeclRefExpr(clang::DeclRefExpr *expr) {
    AddReferencedDecl(expr->getDecl());
    if (!expr->isTypeDependent() && !expr->isValueDependent()) {
      paths_.insert(IndexPath(
          false, Matcher::ExprKey(std::format(
                     "{}{}",
                     clang::UnaryOperator::getOpcodeStr(clang::UO_AddrOf).str(),
                     Printer::ToString(ctx_, expr)))));
    }
    return true;
  }

  bool VisitMemberExpr(clang::MemberExpr *expr) {
    AddReferencedDecl(expr->getMemberDecl());
    return true;
  }

  bool VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
    AddReferencedDecl(expr->getConstructor());
    return true;
  }

  bool VisitEnumDecl(clang::EnumDecl *decl) {
    AddType(decl->getIntegerType());
    AddType(decl->getPromotionType());
    return true;
  }

  bool VisitTypedefNameDecl(clang::TypedefNameDecl *decl) {
    AddType(decl->getUnderlyingType());
    return true;
  }

  bool VisitTypeLoc(clang::TypeLoc loc) {
    AddType(loc.getType());
    return true;
  }

private:
  void AddReferencedDecl(const clang::ValueDecl *decl) {
    AddType(decl->getType());
    if (const auto *fn = llvm::dyn_cast<clang::FunctionDecl>(decl)) {
      for (const auto *redecl : fn->redecls()) {
        AddType(redecl->getReturnType());
        for (const auto *param : redecl->parameters()) {
          AddType(param->getType());
        }
      }
    }
  }

  void AddType(clang::QualType type) {
    if (type.isNull() || type->isDependentType() ||
        !seen_.insert(type.getAsOpaquePtr()).second) {
      return;
    }
    AddTypeKeys(type);
    if (!type->isReferenceType() && !type->isPlaceholderType()) {
      AddTypeKeys(ctx_.getPointerType(type));
      AddTypeKeys(ctx_.getPointerType(type.withConst()));
    }
    if (const auto *enum_decl = type->getAsEnumDecl()) {
      AddType(enum_decl->getIntegerType());
    }

    AddType(type.getUnqualifiedType());
    AddType(type.getCanonicalType());
    AddType(type.getSingleStepDesugaredType(ctx_));
    if (type->isAnyPointerType() || type->isReferenceType() ||
        type->isMemberPointerType()) {
      AddType(type->getPointeeType());
    }
    if (const auto *array = ctx_.getAsArrayType(type)) {
      AddType(array->getElementType());
    }
    if (const auto *fn = type->getAs<clang::FunctionType>()) {
      AddType(fn->getReturnType());
      if (const auto *proto = llvm::dyn_cast<clang::FunctionProtoType>(fn)) {
        for (auto param : proto->param_types()) {
          AddType(param);
        }
      }
    }
    if (const auto *spec = type->getAs<clang::TemplateSpecializationType>()) {
      AddTemplateArgs(spec->template_arguments());
    }
    if (auto *record = type->getAsCXXRecordDecl()) {
      if (const auto *spec =
              llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(record)) {
        AddTemplateArgs(spec->getTemplateArgs().asArray());
      }
      AddSpecialMembers(record);
    }
  }

  void AddSpecialMembers(clang::CXXRecordDecl *record) {
    if (!record->hasDefinition() ||
        !ctx_.getSourceManager().isInSystemHeader(record->getLocation())) {
      return;
    }
    sema_.ForceDeclarationOfImplicitMembers(record->getDefinition());
    for (const auto *method : record->getDefinition()->methods()) {
      if (llvm::isa<clang::CXXConstructorDecl>(method) ||
          llvm::isa<clang::CXXDestructorDecl>(method) ||
          method->isCopyAssignmentOperator() ||
          method->isMoveAssignmentOperator()) {
        paths_.insert(IndexPath(
            false, Matcher::ExprKey(Printer::ToString(ctx_, method))));
      }
    }
  }

  void AddTypeKeys(clang::QualType type) {
    for (const auto &spelling : Printer::ToStringCandidates(ctx_, type)) {
      paths_.insert(IndexPath(true, Matcher::TypeKey(spelling)));
    }
    paths_.insert(
        IndexPath(true, Matcher::TypeKey(Printer::ToString(ctx_, type))));
  }

  void AddTemplateArgs(llvm::ArrayRef<clang::TemplateArgument> args) {
    for (const auto &arg : args) {
      if (arg.getKind() == clang::TemplateArgument::Type) {
        AddType(arg.getAsType());
      } else if (arg.getKind() == clang::TemplateArgument::Pack) {
        AddTemplateArgs(arg.pack_elements());
      }
    }
  }

  clang::ASTContext &ctx_;
  clang::Sema &sema_;
  std::set<std::string> &paths_;
  llvm::DenseSet<const void *> seen_;
};

void AppendIncludes(const fs::path &dir, std::string &out) {
  std::set<fs::path> files;
  std::error_code ec;
  for (fs::directory_iterator it(dir, ec), end; !ec && it != end;
       it.increment(ec)) {
    if (it->is_regular_file()) {
      files.insert(it->path());
    }
  }
  for (const auto &file : files) {
    out += std::format("#include \"{}\"\n", file.string());
  }
}

std::string BuildRulesBuffer(const fs::path &index_dir,
                             const std::set<std::string> &paths, bool is_cxx) {
  std::string out;
  for (const char *lang : {"c", "cpp"}) {
    if (!is_cxx && lang == std::string("cpp")) {
      continue;
    }
    auto root = index_dir / lang;
    AppendIncludes(root / kAllName, out);
    for (const auto &path : paths) {
      AppendIncludes(root / path, out);
    }
  }
  return out;
}

} // namespace

std::string IndexPath(bool is_type, const std::string &key) {
  std::string out = is_type ? "type/" : "expr/";
  for (size_t i = 0; i < key.size(); ++i) {
    unsigned char c = key[i];
    if (c == ':' && i + 1 < key.size() && key[i + 1] == ':') {
      out += '/';
      ++i;
    } else if (std::isalnum(c) || c == '_') {
      out += c;
    } else {
      out += std::format("-{:02x}", c);
    }
  }
  return out;
}

void PragmaHandler::HandlePragma(clang::Preprocessor &PP,
                                 clang::PragmaIntroducer introducer,
                                 clang::Token &tok) {
  while (tok.isNot(clang::tok::eod)) {
    PP.Lex(tok);
  }

  auto &ctx = CI_.getASTContext();
  auto &src_mgr = ctx.getSourceManager();

  CI_.getSema().PerformPendingInstantiations();

  std::vector<clang::Decl *> user_decls;
  for (auto *decl : ctx.getTranslationUnitDecl()->decls()) {
    if (!src_mgr.isInSystemHeader(decl->getLocation())) {
      user_decls.push_back(decl);
    }
  }
  ctx.setTraversalScope(user_decls);
  std::set<std::string> selected;
  ImplicitMemberDefiner definer(CI_.getSema());
  for (auto *decl : user_decls) {
    definer.TraverseDecl(decl);
  }
  CI_.getSema().PerformPendingInstantiations();
  KeyCollector collector(CI_.getSema(), selected);
  for (auto *decl : user_decls) {
    collector.TraverseDecl(decl);
  }
  ctx.setTraversalScope({ctx.getTranslationUnitDecl()});

  auto text = BuildRulesBuffer(fs::path(rules_dir_) / kIndexDirName, selected,
                               ctx.getLangOpts().CPlusPlus);
  log() << "rules loaded for this translation unit:\n" << text;
  rules_file_ = src_mgr.createFileID(
      llvm::MemoryBuffer::getMemBufferCopy(text, "<cpp2rust-rules>"),
      clang::SrcMgr::C_System);
  CI_.getDiagnostics().setSuppressAllDiagnostics(true);
  PP.EnterSourceFile(rules_file_, nullptr, tok.getLocation());
}

} // namespace cpp2rust::RulesLoader
