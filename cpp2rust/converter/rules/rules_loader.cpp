// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/rules_loader.h"

#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Lex/Lexer.h>
#include <clang/Lex/Preprocessor.h>
#include <clang/Sema/Sema.h>
#include <llvm/ADT/DenseSet.h>
#include <llvm/Support/MemoryBuffer.h>

#include <algorithm>
#include <cctype>
#include <filesystem>
#include <format>
#include <set>
#include <vector>

#include "converter/converter_lib.h"
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
    AddKey(ExprKey(ctx_, expr));
    AddType(expr->getType());
    return true;
  }

  bool VisitValueDecl(clang::ValueDecl *decl) {
    AddType(decl->getType());
    return true;
  }

  bool VisitDeclRefExpr(clang::DeclRefExpr *expr) {
    AddReferencedDecl(expr->getDecl());
    return true;
  }

  bool VisitMemberExpr(clang::MemberExpr *expr) {
    AddReferencedDecl(expr->getMemberDecl());
    auto object = expr->getBase()->getType();
    if (expr->isArrow()) {
      object = object->getPointeeType();
    }
    AddMemberKey(object, MemberName(expr->getMemberDecl()->getDeclName()));
    return true;
  }

  bool VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) {
    if (auto method = llvm::dyn_cast_or_null<clang::CXXMethodDecl>(
            expr->getDirectCallee());
        method && expr->getNumArgs() > 0) {
      AddMemberKey(expr->getArg(0)->getType(),
                   MemberName(method->getDeclName()));
    }
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
  void AddKey(const std::string &key) {
    if (!key.empty()) {
      paths_.insert(IndexPath(key));
    }
  }

  void AddMemberKey(clang::QualType object, const std::string &name) {
    if (object.isNull() || object->isDependentType()) {
      return;
    }
    auto record = object.getNonReferenceType()->getAsCXXRecordDecl();
    if (!record ||
        !ctx_.getSourceManager().isInSystemHeader(record->getLocation())) {
      return;
    }
    AddKey(MemberKey(ClassKey(record), name));
  }

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
    AddKey(TypeKey(type));
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
        AddKey(FunctionKey(method));
      }
    }
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

std::vector<fs::path> ListFiles(const fs::path &dir) {
  std::vector<fs::path> files;
  std::error_code ec;
  for (fs::directory_iterator it(dir, ec), end; !ec && it != end;
       it.increment(ec)) {
    if (it->is_regular_file()) {
      files.push_back(it->path());
    }
  }
  std::ranges::sort(files);
  return files;
}

std::string BuildRulesBuffer(const fs::path &index_dir,
                             const std::set<std::string> &paths, bool is_cxx) {
  std::string out;
  for (const char *lang : {"c", "cpp"}) {
    if (!is_cxx && lang == std::string("cpp")) {
      continue;
    }
    for (const auto &path : paths) {
      for (const auto &file : ListFiles(index_dir / lang / path)) {
        out += std::format("#include \"{}\"\n", file.string());
      }
    }
  }
  return out;
}

} // namespace

std::string IndexPath(const std::string &key) {
  std::string out;
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

std::string ClassKey(const clang::NamedDecl *decl) {
  if (auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(decl)) {
    auto tag = alias->getUnderlyingType()->getAsTagDecl();
    if (!tag) {
      return {};
    }
    decl = tag;
  }
  if (auto spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    decl = spec->getSpecializedTemplate();
  }
  if (auto tag = llvm::dyn_cast<clang::TagDecl>(decl);
      tag && !tag->getIdentifier()) {
    decl = tag->getTypedefNameForAnonDecl();
    if (!decl) {
      return {};
    }
  }
  return decl->getQualifiedNameAsString();
}

std::string MemberName(clang::DeclarationName name) {
  if (name.getNameKind() == clang::DeclarationName::CXXConversionFunctionName) {
    return "operator";
  }
  return name.getAsString();
}

std::string MemberKey(const std::string &class_key, const std::string &name) {
  if (class_key.empty()) {
    return {};
  }
  return std::format("{}::{}", class_key, name);
}

std::string FunctionKey(const clang::FunctionDecl *decl) {
  if (auto method = llvm::dyn_cast<clang::CXXMethodDecl>(decl)) {
    auto record = method->getParent();
    if (llvm::isa<clang::CXXConstructorDecl>(method)) {
      return MemberKey(ClassKey(record), record->getNameAsString());
    }
    return MemberKey(ClassKey(record), MemberName(method->getDeclName()));
  }
  return decl->getQualifiedNameAsString();
}

std::string DeclKey(const clang::NamedDecl *decl) {
  if (auto fn = decl->getAsFunction()) {
    return FunctionKey(fn);
  }
  if (auto record = llvm::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
    return MemberKey(ClassKey(record), MemberName(decl->getDeclName()));
  }
  return decl->getQualifiedNameAsString();
}

std::string ExprKey(clang::ASTContext &ctx, const clang::Expr *expr) {
  expr = expr->IgnoreParenImpCasts();
  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    return clang::Lexer::getImmediateMacroName(
               expr->getBeginLoc(), ctx.getSourceManager(), ctx.getLangOpts())
        .str();
  }
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (auto callee = call->getDirectCallee()) {
      return FunctionKey(callee);
    }
    return {};
  }
  if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    return FunctionKey(construct->getConstructor());
  }
  if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    return DeclKey(member->getMemberDecl());
  }
  if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return DeclKey(ref->getDecl());
  }
  if (auto unary = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    return ExprKey(ctx, unary->getSubExpr());
  }
  return {};
}

std::string TypeKey(clang::QualType type) {
  if (auto decltype_type =
          llvm::dyn_cast<clang::DecltypeType>(type.getTypePtr())) {
    type = decltype_type->getUnderlyingType();
  }
  if (auto typeof_type =
          llvm::dyn_cast<clang::TypeOfExprType>(type.getTypePtr())) {
    type = typeof_type->getUnderlyingExpr()->getType();
  }
  if (auto alias = type->getAs<clang::TypedefType>();
      alias && type.getCanonicalType()->isBuiltinType()) {
    return DeclKey(alias->getDecl());
  }
  if (auto sugar = type->getAs<clang::PredefinedSugarType>()) {
    return sugar->getIdentifier()->getName().str();
  }
  if (auto tag = type->getAsTagDecl()) {
    return ClassKey(tag);
  }
  return {};
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
  auto rules_file = src_mgr.createFileID(
      llvm::MemoryBuffer::getMemBufferCopy(text, "<cpp2rust-rules>"),
      clang::SrcMgr::C_System);
  PP.EnterSourceFile(rules_file, nullptr, tok.getLocation());
}

} // namespace cpp2rust::RulesLoader
