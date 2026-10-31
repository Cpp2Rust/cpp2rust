// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/decls.h"

#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/Sema/Template.h>
#include <clang/Sema/TemplateDeduction.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/ADT/SmallVector.h>

#include <cassert>
#include <optional>
#include <vector>

#include "converter/converter_lib.h"

namespace cpp2rust::RuleDecls {

const clang::Expr *SkipImplicit(const clang::Expr *expr) {
  while (true) {
    expr = expr->IgnoreParens();
    if (auto full = llvm::dyn_cast<clang::FullExpr>(expr)) {
      expr = full->getSubExpr();
    } else if (auto temporary =
                   llvm::dyn_cast<clang::MaterializeTemporaryExpr>(expr)) {
      expr = temporary->getSubExpr();
    } else if (auto bind = llvm::dyn_cast<clang::CXXBindTemporaryExpr>(expr)) {
      expr = bind->getSubExpr();
    } else if (auto cast = llvm::dyn_cast<clang::ImplicitCastExpr>(expr)) {
      expr = cast->getSubExpr();
    } else if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr);
               construct && construct->isElidable()) {
      expr = construct->getArg(0);
    } else {
      return expr;
    }
  }
}

const clang::Expr *GetReturned(const clang::FunctionDecl *rule) {
  auto body = llvm::dyn_cast_or_null<clang::CompoundStmt>(rule->getBody());
  assert(body && body->size() == 1 && "rule body must be a single return");
  auto ret = llvm::cast<clang::ReturnStmt>(*body->body_begin());
  auto returned = ret->getRetValue();
  auto construct =
      llvm::dyn_cast<clang::CXXConstructExpr>(SkipImplicit(returned));
  if (construct && construct->getConstructor()->isCopyOrMoveConstructor() &&
      construct->getConstructor()->isTrivial()) {
    return construct->getArg(0);
  }
  return returned;
}

namespace {
clang::FileID getMacroExpansion(const clang::SourceManager &sm,
                                clang::SourceLocation loc) {
  while (loc.isMacroID() && sm.isMacroArgExpansion(loc)) {
    loc = sm.getImmediateSpellingLoc(loc);
  }
  return loc.isMacroID() ? sm.getFileID(loc) : clang::FileID();
}
} // namespace

const clang::Expr *GetMacroMemberBase(const clang::SourceManager &sm,
                                      const clang::MemberExpr *member) {
  auto file = getMacroExpansion(sm, member->getMemberLoc());
  if (file.isInvalid() ||
      sm.getSLocEntry(file).getExpansion().isFunctionMacroExpansion()) {
    return nullptr;
  }
  const clang::Expr *base = member->getBase();
  while (true) {
    auto inner = llvm::dyn_cast<clang::MemberExpr>(base->IgnoreParenImpCasts());
    if (!inner || getMacroExpansion(sm, inner->getMemberLoc()) != file) {
      return base;
    }
    base = inner->getBase();
  }
}

namespace {

std::vector<clang::TemplateArgument> deducedArguments(
    clang::Sema &sema, clang::TemplateParameterList *params,
    llvm::SmallVectorImpl<clang::DeducedTemplateArgument> &deduced) {
  for (unsigned i = 0; i < deduced.size(); ++i) {
    auto nttp =
        llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(params->getParam(i));
    if (nttp && !nttp->getType()->isDependentType() &&
        deduced[i].getKind() == clang::TemplateArgument::Integral) {
      deduced[i] = clang::TemplateArgument(
          sema.Context, deduced[i].getAsIntegral(), nttp->getType());
    }
  }
  return std::vector<clang::TemplateArgument>(deduced.begin(), deduced.end());
}

} // namespace

std::optional<std::vector<clang::TemplateArgument>>
DeduceFromType(clang::Sema &sema, clang::TemplateParameterList *params,
               clang::QualType pattern, clang::QualType actual) {
  Trial trial(sema);
  llvm::SmallVector<clang::DeducedTemplateArgument, 4> deduced(params->size());
  clang::sema::TemplateDeductionInfo info(params->getTemplateLoc());
  if (sema.DeduceTemplateArguments(params, clang::TemplateArgument(pattern),
                                   clang::TemplateArgument(actual), info,
                                   deduced, /*NumberOfArgumentsMustMatch=*/
                                   true) !=
          clang::TemplateDeductionResult::Success ||
      trial.failed()) {
    return std::nullopt;
  }
  return deducedArguments(sema, params, deduced);
}

std::optional<std::vector<clang::TemplateArgument>>
DeduceFromArguments(clang::Sema &sema, clang::TemplateParameterList *params,
                    llvm::ArrayRef<clang::TemplateArgumentLoc> pattern,
                    llvm::ArrayRef<clang::TemplateArgument> actual) {
  Trial trial(sema);
  llvm::SmallVector<clang::TemplateArgument, 4> written;
  for (const auto &arg : pattern) {
    written.push_back(arg.getArgument());
  }
  llvm::SmallVector<clang::DeducedTemplateArgument, 4> deduced(params->size());
  clang::sema::TemplateDeductionInfo info(params->getTemplateLoc());
  if (sema.DeduceTemplateArguments(params, written, actual, info, deduced,
                                   /*NumberOfArgumentsMustMatch=*/false) !=
          clang::TemplateDeductionResult::Success ||
      trial.failed()) {
    return std::nullopt;
  }
  return deducedArguments(sema, params, deduced);
}

namespace {

const clang::TemplateSpecializationType *
getMemberParentClass(clang::QualType pattern, clang::ClassTemplateDecl *&tmpl) {
  auto member = pattern->getAs<clang::DependentNameType>();
  if (!member || member->getQualifier().getKind() !=
                     clang::NestedNameSpecifier::Kind::Type) {
    return nullptr;
  }
  auto spec = member->getQualifier()
                  .getAsType()
                  ->getAs<clang::TemplateSpecializationType>();
  tmpl = spec ? llvm::dyn_cast_or_null<clang::ClassTemplateDecl>(
                    spec->getTemplateName().getAsTemplateDecl())
              : nullptr;
  return tmpl && !tmpl->getTemplateParameters()->hasParameterPack() ? spec
                                                                    : nullptr;
}

std::optional<std::vector<clang::TemplateArgument>>
getDefaultedArgumentsOfRule(clang::Sema &sema, const clang::TemplateDecl *rule,
                            clang::ClassTemplateDecl *tmpl,
                            const clang::TemplateSpecializationType *spec) {
  auto params = rule->getTemplateParameters();
  auto loc = params->getTemplateLoc();
  clang::TemplateArgumentListInfo written(loc, loc);
  llvm::SmallVector<std::optional<unsigned>, 4> positions(params->size());
  for (auto [i, arg] : llvm::enumerate(spec->template_arguments())) {
    auto parm = arg.getKind() == clang::TemplateArgument::Type
                    ? arg.getAsType()->getAs<clang::TemplateTypeParmType>()
                    : nullptr;
    if (parm && parm->getDepth() == params->getDepth() &&
        !parm->isParameterPack()) {
      if (positions[parm->getIndex()]) {
        return std::nullopt;
      }
      positions[parm->getIndex()] = i;
      continue;
    }
    if (arg.isDependent() || written.size() != i) {
      return std::nullopt;
    }
    written.addArgument(sema.getTrivialTemplateArgumentLoc(
        arg, arg.getNonTypeTemplateArgumentType(), loc));
  }
  if (!llvm::all_of(positions, [](const std::optional<unsigned> &position) {
        return position.has_value();
      })) {
    return std::nullopt;
  }
  Trial trial(sema);
  clang::Sema::CheckTemplateArgumentInfo info;
  if (sema.CheckTemplateArgumentList(tmpl, loc, written, /*DefaultArgs=*/{},
                                     /*PartialTemplateArgs=*/false, info) ||
      trial.failed()) {
    return std::nullopt;
  }
  std::vector<clang::TemplateArgument> args(params->size());
  for (unsigned i = 0; i < params->size(); ++i) {
    args[i] = info.CanonicalConverted[*positions[i]];
  }
  return args;
}

bool resolvesTo(clang::Sema &sema, const clang::TemplateDecl *rule,
                llvm::ArrayRef<clang::TemplateArgument> args,
                clang::QualType pattern, clang::QualType actual) {
  Trial trial(sema);
  clang::MultiLevelTemplateArgumentList bound(
      const_cast<clang::TemplateDecl *>(rule), args, /*Final=*/true);
  auto resolved = sema.SubstType(
      pattern, bound, rule->getTemplateParameters()->getTemplateLoc(),
      clang::DeclarationName());
  return !resolved.isNull() && !trial.failed() &&
         sema.Context.hasSameUnqualifiedType(resolved, actual);
}

} // namespace

std::optional<std::vector<clang::TemplateArgument>>
DeduceMemberType(clang::Sema &sema, const clang::TemplateDecl *rule,
                 clang::QualType pattern, clang::QualType use) {
  auto &ctx = sema.Context;
  clang::ClassTemplateDecl *tmpl = nullptr;
  auto spec = getMemberParentClass(pattern, tmpl);
  if (!spec) {
    return std::nullopt;
  }
  auto member = pattern->getAs<clang::DependentNameType>();
  auto params = rule->getTemplateParameters();
  if (auto alias = SpelledTypedef(ctx, use)) {
    if (alias->getIdentifier() != member->getIdentifier()) {
      return std::nullopt;
    }
    auto owner = llvm::dyn_cast<clang::CXXRecordDecl>(alias->getDeclContext());
    if (auto args = owner
                        ? DeduceFromType(sema, params, clang::QualType(spec, 0),
                                         ctx.getCanonicalTagType(owner))
                        : std::nullopt) {
      return args;
    }
  }
  auto args = getDefaultedArgumentsOfRule(sema, rule, tmpl, spec);
  if (!args || !resolvesTo(sema, rule, *args, pattern, use)) {
    return std::nullopt;
  }
  return args;
}

bool IsInternal(const clang::NamedDecl *decl) {
  if (decl->getAccess() == clang::AS_private ||
      decl->getAccess() == clang::AS_protected) {
    return true;
  }
  auto &opts = decl->getASTContext().getLangOpts();
  if (decl->isReserved(opts) != clang::ReservedIdentifierStatus::NotReserved) {
    return true;
  }
  for (auto context = decl->getDeclContext(); context;
       context = context->getParent()) {
    if (context->isInlineNamespace()) {
      continue;
    }
    auto named = llvm::dyn_cast<clang::NamedDecl>(context);
    if (named && named->getIdentifier() &&
        named->isReserved(opts) !=
            clang::ReservedIdentifierStatus::NotReserved) {
      return true;
    }
  }
  return false;
}

clang::QualType PeelSugar(clang::ASTContext &ctx, clang::QualType type) {
  while (!llvm::isa<clang::TypedefType, clang::PredefinedSugarType>(
             type.getTypePtr()) &&
         type != type.getSingleStepDesugaredType(ctx)) {
    type = type.getSingleStepDesugaredType(ctx);
  }
  return type;
}

const clang::Type *GetSugar(clang::QualType type) {
  if (auto typeof_type =
          llvm::dyn_cast<clang::TypeOfExprType>(type.getTypePtr())) {
    type = typeof_type->getUnderlyingExpr()->getType();
  }
  if (llvm::isa<clang::TypedefType, clang::PredefinedSugarType>(
          type.getTypePtr())) {
    return type.getTypePtr();
  }
  return nullptr;
}

clang::QualType SkipInternalTypedefs(clang::ASTContext &ctx,
                                     clang::QualType use) {
  auto cvr = use.getLocalCVRQualifiers();
  auto type = PeelSugar(ctx, use);
  while (auto typedef_type =
             llvm::dyn_cast<clang::TypedefType>(type.getTypePtr())) {
    auto decl = typedef_type->getDecl();
    if (!IsInternal(decl) && !IsUserDefinedDecl(decl)) {
      break;
    }
    type = PeelSugar(ctx, typedef_type->desugar());
    cvr |= type.getLocalCVRQualifiers();
  }
  return type.withCVRQualifiers(cvr);
}

const clang::TypedefNameDecl *SpelledTypedef(clang::ASTContext &ctx,
                                             clang::QualType use) {
  auto sugar = GetSugar(PeelSugar(ctx, use));
  auto typedef_type = llvm::dyn_cast_or_null<clang::TypedefType>(sugar);
  return typedef_type ? typedef_type->getDecl() : nullptr;
}

} // namespace cpp2rust::RuleDecls
