// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/rule_keys.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Lex/Lexer.h>
#include <llvm/ADT/STLExtras.h>

#include <cassert>
#include <optional>
#include <vector>

#include "converter/rules/decls.h"

namespace cpp2rust::RuleKeys {

namespace {

const clang::Decl *getParent(clang::QualType type) {
  type = type.getNonReferenceType();
  if (type->isPointerType()) {
    type = type->getPointeeType();
  }
  if (auto spec = type->getAs<clang::TemplateSpecializationType>()) {
    if (auto tmpl = llvm::dyn_cast_or_null<clang::ClassTemplateDecl>(
            spec->getTemplateName().getAsTemplateDecl())) {
      return tmpl->getCanonicalDecl();
    }
    return nullptr;
  }
  auto record = type->getAsCXXRecordDecl();
  if (!record) {
    return nullptr;
  }
  if (auto spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(record)) {
    return spec->getSpecializedTemplate()->getCanonicalDecl();
  }
  return record->getCanonicalDecl();
}

const clang::Decl *getParent(const clang::Decl *decl) {
  if (auto record =
          llvm::dyn_cast<clang::CXXRecordDecl>(decl->getDeclContext())) {
    return getParent(decl->getASTContext().getCanonicalTagType(record));
  }
  return nullptr;
}

std::vector<clang::QualType> resolveMemberType(clang::QualType type) {
  auto member = type->getAs<clang::DependentNameType>();
  if (!member) {
    return {type};
  }
  auto qualifier = member->getQualifier().getAsType();
  assert(qualifier && "member type is not qualified by a type");
  std::vector<clang::QualType> resolved;
  for (auto owner : resolveMemberType(clang::QualType(qualifier, 0))) {
    auto spec = owner->getAs<clang::TemplateSpecializationType>();
    auto tmpl = spec ? llvm::dyn_cast_or_null<clang::ClassTemplateDecl>(
                           spec->getTemplateName().getAsTemplateDecl())
                     : nullptr;
    assert(tmpl && "member type is not qualified by a class template");
    llvm::SmallVector<clang::ClassTemplatePartialSpecializationDecl *> partials;
    tmpl->getPartialSpecializations(partials);
    llvm::SmallVector<clang::CXXRecordDecl *> records{tmpl->getTemplatedDecl()};
    records.append(partials.begin(), partials.end());
    for (auto record : records) {
      if (!record->hasDefinition()) {
        continue;
      }
      for (auto decl :
           record->getDefinition()->lookup(member->getIdentifier())) {
        if (auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(decl)) {
          auto types = resolveMemberType(alias->getUnderlyingType());
          resolved.insert(resolved.end(), types.begin(), types.end());
        }
      }
    }
  }
  return resolved;
}

ExprKey getOperatorKey(clang::ASTContext &ctx,
                       clang::OverloadedOperatorKind op) {
  return Unqualified{ctx.DeclarationNames.getCXXOperatorName(op)};
}

std::optional<ExprKey> getConstructorKey(const clang::Decl *parent) {
  if (!parent) {
    return std::nullopt;
  }
  return Constructor{parent};
}

std::optional<ExprKey> getMemberKey(const clang::Decl *parent,
                                    clang::DeclarationName name) {
  if (name.getNameKind() == clang::DeclarationName::CXXConversionFunctionName) {
    return getConstructorKey(parent);
  }
  if (!parent) {
    return Unqualified{name};
  }
  return Member{parent, name};
}

std::optional<ExprKey> getDeclKey(const clang::NamedDecl *decl) {
  if (auto ctor = llvm::dyn_cast<clang::CXXConstructorDecl>(decl)) {
    return getConstructorKey(getParent(ctor));
  }
  return getMemberKey(getParent(decl), decl->getDeclName());
}

std::optional<ExprKey> getExprKey(clang::ASTContext &ctx,
                                  const clang::Expr *expr) {
  expr = RuleDecls::SkipImplicit(expr);
  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    auto macro = clang::Lexer::getImmediateMacroName(
        expr->getBeginLoc(), ctx.getSourceManager(), ctx.getLangOpts());
    return Unqualified{clang::DeclarationName(&ctx.Idents.get(macro))};
  }
  if (auto rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    return getOperatorKey(ctx, clang::BinaryOperator::getOverloadedOperator(
                                   rewritten->getOperator()));
  }
  if (auto cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    return getExprKey(ctx, cast->getSubExpr());
  }
  if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    return getDeclKey(construct->getConstructor());
  }
  if (auto construct =
          llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(expr)) {
    return getConstructorKey(getParent(construct->getTypeAsWritten()));
  }
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (auto callee = call->getDirectCallee()) {
      return getDeclKey(callee);
    }
    return getExprKey(ctx, call->getCallee());
  }
  if (auto member = llvm::dyn_cast<clang::UnresolvedMemberExpr>(expr)) {
    return getMemberKey(getParent(member->getBaseType()),
                        member->getMemberName());
  }
  if (auto member = llvm::dyn_cast<clang::CXXDependentScopeMemberExpr>(expr)) {
    return getMemberKey(getParent(member->getBaseType()), member->getMember());
  }
  if (auto lookup = llvm::dyn_cast<clang::UnresolvedLookupExpr>(expr)) {
    return Unqualified{lookup->getName()};
  }
  if (auto ref = llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(expr)) {
    return Unqualified{ref->getDeclName()};
  }
  if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    return getDeclKey(member->getMemberDecl());
  }
  if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return getDeclKey(ref->getDecl());
  }
  if (auto binary = llvm::dyn_cast<clang::BinaryOperator>(expr)) {
    return getOperatorKey(
        ctx, clang::BinaryOperator::getOverloadedOperator(binary->getOpcode()));
  }
  if (auto unary = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    return getOperatorKey(
        ctx, clang::UnaryOperator::getOverloadedOperator(unary->getOpcode()));
  }
  if (llvm::isa<clang::ArraySubscriptExpr>(expr)) {
    return getOperatorKey(ctx, clang::OO_Subscript);
  }
  return std::nullopt;
}

} // namespace

std::optional<ExprKey> GetExprKey(clang::ASTContext &ctx,
                                  const clang::Expr *use) {
  return getExprKey(ctx, use);
}

std::vector<ExprKey> GetExprKeys(clang::ASTContext &ctx,
                                 const clang::NamedDecl *rule) {
  if (auto var = llvm::dyn_cast<clang::VarDecl>(rule)) {
    auto key = getExprKey(ctx, var->getInit());
    assert(key && "expression rule has no key");
    return {*key};
  }
  auto returned =
      RuleDecls::SkipImplicit(RuleDecls::GetReturned(rule->getAsFunction()));
  if (auto construct =
          llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(returned)) {
    std::vector<ExprKey> keys;
    for (auto type : resolveMemberType(construct->getTypeAsWritten())) {
      auto key = getConstructorKey(getParent(type));
      assert(key && "expression rule has no key");
      if (!llvm::is_contained(keys, *key)) {
        keys.push_back(*key);
      }
    }
    return keys;
  }
  auto key = getExprKey(ctx, returned);
  assert(key && "expression rule has no key");
  return {*key};
}

std::optional<TypeKey> GetTypeKey(clang::QualType type) {
  type = type.getCanonicalType().getUnqualifiedType();
  if (auto parent = getParent(type)) {
    return parent;
  }
  if (auto tag = type->getAsTagDecl()) {
    return tag->getCanonicalDecl();
  }
  if (type->isDependentType()) {
    return std::nullopt;
  }
  return type.getTypePtr();
}

std::vector<TypeKey> GetTypeKeys(const clang::NamedDecl *rule) {
  auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(rule);
  if (auto tmpl = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(rule)) {
    alias = tmpl->getTemplatedDecl();
  }
  assert(alias && "type rule is not an alias");
  auto pattern = alias->getUnderlyingType();
  if (pattern->getAs<clang::TemplateTypeParmType>()) {
    pattern = RuleDecls::EnableIfSameType(rule);
    assert(!pattern.isNull() &&
           "type rule matching any type must be constrained by enable_if");
    std::vector<TypeKey> keys;
    for (auto type : resolveMemberType(pattern)) {
      auto key = GetTypeKey(type);
      assert(key && "type rule has no key");
      if (!llvm::is_contained(keys, *key)) {
        keys.push_back(*key);
      }
    }
    return keys;
  }
  if (auto member = pattern->getAs<clang::DependentNameType>()) {
    auto qualifier = member->getQualifier().getAsType();
    assert(qualifier && "member type is not qualified by a type");
    auto parent = getParent(clang::QualType(qualifier, 0));
    assert(parent && "type rule has no key");
    return {parent};
  }
  auto key = GetTypeKey(pattern);
  assert(key && "type rule has no key");
  return {*key};
}

std::vector<TypeKey> GetTypeKeys(clang::ASTContext &ctx, clang::QualType use) {
  std::vector<TypeKey> keys;
  if (auto key = GetTypeKey(use)) {
    keys.push_back(*key);
  }
  for (auto type = use;; type = type.getSingleStepDesugaredType(ctx)) {
    if (auto typedef_type =
            llvm::dyn_cast<clang::TypedefType>(type.getTypePtr())) {
      if (auto owner = llvm::dyn_cast<clang::CXXRecordDecl>(
              typedef_type->getDecl()->getDeclContext())) {
        auto key = GetTypeKey(ctx.getCanonicalTagType(owner));
        if (key && !llvm::is_contained(keys, *key)) {
          keys.push_back(*key);
        }
      }
    }
    if (type.getSingleStepDesugaredType(ctx) == type) {
      return keys;
    }
  }
}

std::optional<ExprKey> GetDeclKey(const clang::NamedDecl *decl) {
  return getDeclKey(decl);
}

} // namespace cpp2rust::RuleKeys
