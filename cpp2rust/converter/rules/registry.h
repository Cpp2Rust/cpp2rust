#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>
#include <clang/Sema/Sema.h>

#include <string>
#include <unordered_map>
#include <vector>

#include "converter/factory.h"
#include "converter/rules/matcher.h"
#include "converter/translation_rule.h"

namespace cpp2rust::RuleRegistry {
struct ExprRuleDecl {
  const clang::NamedDecl *decl;
  TranslationRule::ExprRule rule;
};

struct TypeRuleDecl {
  const clang::NamedDecl *decl;
  TranslationRule::TypeRule rule;
};

void Load(clang::Sema &sema, Model model);

Matcher::Match<TranslationRule::ExprRule> Search(clang::ASTContext &ctx,
                                                 const clang::Expr *expr);
Matcher::Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                                 clang::QualType qual_type);

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr);
clang::QualType GetInitType(const clang::Expr *expr);

clang::QualType GetGenericType(clang::ASTContext &ctx, const clang::Expr *expr,
                               unsigned n);

clang::QualType GetConversionSourceType(clang::ASTContext &ctx,
                                        const clang::Expr *expr, unsigned to);

clang::QualType GetDeclaredReturnType(const clang::Expr *expr);

bool HasExprRule(const clang::FunctionDecl *decl);
bool IsLibcPassthrough(clang::ASTContext &ctx, const clang::Expr *expr);
bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr);
bool ReturnsReference(const clang::Expr *expr);
bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index);

clang::Expr *BuildParamConversion(clang::ASTContext &ctx,
                                  const clang::Expr *expr, unsigned index,
                                  clang::Expr *arg);

bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type);
bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type);

Model CurrentModel();

} // namespace cpp2rust::RuleRegistry
