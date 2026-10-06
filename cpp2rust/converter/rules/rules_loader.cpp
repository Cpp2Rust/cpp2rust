// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/rules_loader.h"

#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Lex/Lexer.h>
#include <clang/Lex/Preprocessor.h>
#include <clang/Sema/Sema.h>
#include <llvm/ADT/DenseSet.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/Support/Error.h>
#include <llvm/Support/ErrorHandling.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/MemoryBuffer.h>

#include <cassert>
#include <filesystem>
#include <ranges>
#include <string>
#include <unordered_set>
#include <vector>

#include "converter/converter_lib.h"
#include "logging.h"

namespace cpp2rust::RulesLoader {

namespace {

class RuleUsageCollector
    : public clang::RecursiveASTVisitor<RuleUsageCollector> {
public:
  RuleUsageCollector(clang::Sema &sema, std::unordered_set<std::string> &keys)
      : ctx_(sema.Context), sema_(sema), keys_(keys) {}

  bool shouldVisitTemplateInstantiations() const { return true; }

  bool shouldVisitImplicitCode() const { return true; }

  bool TraverseDecl(clang::Decl *decl) {
    if (decl && !llvm::isa<clang::TranslationUnitDecl>(decl) &&
        ctx_.getSourceManager().isInSystemHeader(decl->getLocation())) {
      return true;
    }
    return RecursiveASTVisitor::TraverseDecl(decl);
  }

  bool VisitCXXRecordDecl(clang::CXXRecordDecl *decl) {
    if (decl->isThisDeclarationADefinition() && !decl->isDependentType() &&
        IsConvertibleCXXRecordDecl(decl) &&
        (decl->isStruct() || decl->isClass()) && !decl->isAbstract()) {
      DefineImplicitMembers(sema_, decl);
    }
    return true;
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
      keys_.insert(key);
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
    if (const auto *alias =
            llvm::dyn_cast<clang::TypedefType>(type.getTypePtr())) {
      if (const auto *record = llvm::dyn_cast<clang::RecordDecl>(
              alias->getDecl()->getDeclContext())) {
        AddKey(ClassKey(record));
      }
    }
    if (auto desugared = type.getSingleStepDesugaredType(ctx_);
        desugared != type) {
      if (const auto *spec = llvm::dyn_cast<clang::TemplateSpecializationType>(
              type.getTypePtr())) {
        AddTemplateArgs(spec->template_arguments());
      }
      return AddType(desugared);
    }
    if (type->isBuiltinType() || type->isUndeducedType()) {
      return;
    }
    if (type->isPointerType() || type->isReferenceType()) {
      return AddType(type->getPointeeType());
    }
    if (const auto *array = ctx_.getAsArrayType(type)) {
      return AddType(array->getElementType());
    }
    if (const auto *vector = type->getAs<clang::VectorType>()) {
      return AddType(vector->getElementType());
    }
    if (const auto *fn = type->getAs<clang::FunctionType>()) {
      AddType(fn->getReturnType());
      if (const auto *proto = llvm::dyn_cast<clang::FunctionProtoType>(fn)) {
        for (auto param : proto->param_types()) {
          AddType(param);
        }
      }
      return;
    }
    if (const auto *enum_decl = type->getAsEnumDecl()) {
      return AddType(enum_decl->getIntegerType());
    }
    if (type->isRecordType()) {
      if (auto *record = type->getAsCXXRecordDecl()) {
        if (const auto *spec =
                llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                    record)) {
          AddTemplateArgs(spec->getTemplateArgs().asArray());
        }
        AddSpecialMembers(record);
      }
      return;
    }
    type.dump();
    assert(0 && "type is not scanned");
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
      switch (arg.getKind()) {
      case clang::TemplateArgument::Type:
        AddType(arg.getAsType());
        break;
      case clang::TemplateArgument::Pack:
        AddTemplateArgs(arg.pack_elements());
        break;
      case clang::TemplateArgument::Null:
      case clang::TemplateArgument::Integral:
      case clang::TemplateArgument::NullPtr:
      case clang::TemplateArgument::Expression:
      case clang::TemplateArgument::Template:
        break;
      default:
        assert(0 && "template argument kind is not scanned");
        break;
      }
    }
  }

  clang::ASTContext &ctx_;
  clang::Sema &sema_;
  std::unordered_set<std::string> &keys_;
  llvm::DenseSet<const void *> seen_;
};

llvm::json::Object ReadIndex(const std::filesystem::path &path) {
  auto buf = llvm::MemoryBuffer::getFile(path.string());
  if (!buf) {
    llvm::errs() << "Missing " << path.string() << ", run cpp-rule-indexer\n";
    llvm::report_fatal_error("cannot read the rule index");
  }
  auto parsed = llvm::json::parse((*buf)->getBuffer());
  if (!parsed) {
    llvm::errs() << "Failed to parse rule index: " << path.string() << ": "
                 << llvm::toString(parsed.takeError()) << '\n';
    llvm::report_fatal_error("cannot parse the rule index");
  }
  auto index = parsed->getAsObject();
  if (!index) {
    llvm::errs() << "Rule index is not an object: " << path.string() << '\n';
    llvm::report_fatal_error("cannot parse the rule index");
  }
  return std::move(*index);
}

const llvm::json::Object &CIndex() {
  static const llvm::json::Object index =
      ReadIndex(std::filesystem::path(RULES_INDEX_DIR) / "c.json");
  return index;
}

const llvm::json::Object &CxxIndex() {
  static const llvm::json::Object index =
      ReadIndex(std::filesystem::path(RULES_INDEX_DIR) / "cpp.json");
  return index;
}

std::string BuildRulesEpilogue(const std::unordered_set<std::string> &keys,
                               bool is_cxx) {
  const auto &index = is_cxx ? CxxIndex() : CIndex();
  auto common = index.getString("common");
  std::string out;
  if (!common->empty()) {
    out += "namespace cpp2rust_rules {\n" + common->str() + "}\n";
  }
  auto rules = index.getObject("rules");
  for (const auto &key : keys) {
    auto entries = rules->getArray(key);
    if (!entries) {
      continue;
    }
    for (const auto &entry : *entries) {
      auto rule = entry.getAsObject();
      if (auto required = rule->getArray("requires");
          required &&
          !llvm::all_of(*required, [&](const llvm::json::Value &key) {
            return keys.contains(key.getAsString()->str());
          })) {
        continue;
      }
      out += *rule->getString("text");
    }
  }
  return out;
}

} // namespace

static std::string NameKey(const clang::NamedDecl *decl) {
  std::vector<std::string> scopes;
  for (auto scope = decl->getDeclContext(); scope; scope = scope->getParent()) {
    if (auto ns = llvm::dyn_cast<clang::NamespaceDecl>(scope)) {
      if (!ns->isInline() && !ns->isAnonymousNamespace()) {
        scopes.push_back(ns->getNameAsString());
      }
    } else if (auto record = llvm::dyn_cast<clang::RecordDecl>(scope)) {
      scopes.push_back(record->getNameAsString());
    } else if (auto enum_decl = llvm::dyn_cast<clang::EnumDecl>(scope);
               enum_decl && enum_decl->isScoped()) {
      scopes.push_back(enum_decl->getNameAsString());
    }
  }
  std::string key;
  for (const auto &scope : scopes | std::views::reverse) {
    key += scope + '/';
  }
  return key + decl->getNameAsString();
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
  return NameKey(decl);
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
  return class_key + '/' + name;
}

std::string ConstructorKey(const std::string &class_key) {
  return MemberKey(class_key, class_key.substr(class_key.rfind('/') + 1));
}

std::string FunctionKey(const clang::FunctionDecl *decl) {
  if (const clang::FunctionDecl *definition = nullptr;
      decl->isDefined(definition) &&
      !decl->getASTContext().getSourceManager().isInSystemHeader(
          definition->getLocation())) {
    return {};
  }
  if (auto method = llvm::dyn_cast<clang::CXXMethodDecl>(decl)) {
    auto class_key = ClassKey(method->getParent());
    if (llvm::isa<clang::CXXConstructorDecl>(method)) {
      return ConstructorKey(class_key);
    }
    return MemberKey(class_key, MemberName(method->getDeclName()));
  }
  return NameKey(decl);
}

std::string DeclKey(const clang::NamedDecl *decl) {
  if (auto fn = decl->getAsFunction()) {
    return FunctionKey(fn);
  }
  if (auto record = llvm::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
    return MemberKey(ClassKey(record), MemberName(decl->getDeclName()));
  }
  return NameKey(decl);
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

  std::unordered_set<std::string> keys;
  RuleUsageCollector collector(CI_.getSema(), keys);
  collector.TraverseDecl(ctx.getTranslationUnitDecl());

  auto text = BuildRulesEpilogue(keys, ctx.getLangOpts().CPlusPlus);
  log() << "rules loaded for this translation unit:\n" << text;
  auto rules_file = src_mgr.createFileID(
      llvm::MemoryBuffer::getMemBufferCopy(text, "<cpp2rust-rules>"),
      clang::SrcMgr::C_System);
  PP.EnterSourceFile(rules_file, nullptr, tok.getLocation());
}

} // namespace cpp2rust::RulesLoader
