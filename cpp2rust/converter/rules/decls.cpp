// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/decls.h"

#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/Sema/Template.h>
#include <clang/Sema/TemplateDeduction.h>
#include <llvm/ADT/SmallVector.h>

#include <cassert>
#include <optional>
#include <vector>

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

clang::QualType EnableIfSameType(const clang::Decl *rule) {
  auto tmpl = llvm::dyn_cast<clang::TemplateDecl>(rule);
  if (!tmpl) {
    return {};
  }
  for (auto param : *tmpl->getTemplateParameters()) {
    auto type_param = llvm::dyn_cast<clang::TemplateTypeParmDecl>(param);
    if (!type_param || type_param->getIdentifier() ||
        !type_param->hasDefaultArgument()) {
      continue;
    }
    auto same = type_param->getDefaultArgument()
                    .getArgument()
                    .getAsType()
                    ->getAs<clang::TemplateSpecializationType>();
    if (!same || !same->isTypeAlias() ||
        same->getTemplateName().getAsTemplateDecl()->getName() != "SameAs") {
      continue;
    }
    return same->template_arguments()[1].getAsType();
  }
  return {};
}

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
  return std::vector<clang::TemplateArgument>(deduced.begin(), deduced.end());
}

} // namespace cpp2rust::RuleDecls
