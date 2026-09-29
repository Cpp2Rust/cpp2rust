#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <ranges>
#include <string>
#include <unordered_map>
#include <vector>

#include "converter/factory.h"
#include "converter/rules/matcher.h"
#include "converter/translation_rule.h"

namespace cpp2rust::RuleRegistry {
using ExprRuleMap =
    std::unordered_multimap<std::string, TranslationRule::ExprRule>;
using TypeRuleMap =
    std::unordered_multimap<std::string, TranslationRule::TypeRule>;

std::ranges::subrange<ExprRuleMap::iterator>
ExprCandidates(const std::string &key);
std::ranges::subrange<TypeRuleMap::iterator>
TypeCandidates(const std::string &key);

Matcher::Match<TranslationRule::ExprRule> Search(clang::ASTContext &ctx,
                                                 const clang::Expr *expr);
Matcher::Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                                 clang::QualType qual_type);

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr);
bool IsLibcPassthrough(clang::ASTContext &ctx, const clang::Expr *expr);
bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr);
bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index);
bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type);
bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type);
const std::vector<std::string> *MappedDerives(clang::ASTContext &ctx,
                                              clang::QualType qual_type);
void SetDerives(clang::ASTContext &ctx, clang::QualType qual_type,
                std::vector<std::string> derives);

Model CurrentModel();

void Load(Model model, const std::string &rules_dir);
void AddRuleForUserDefinedType(clang::ASTContext &ctx, clang::NamedDecl *decl);
} // namespace cpp2rust::RuleRegistry
