#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <string>
#include <utility>

#include "converter/factory.h"
#include "converter/rules/matching.h"
#include "converter/translation_rule.h"

namespace cpp2rust::RuleRegistry {
template <typename Rule> using Match = std::pair<Rule *, Matching::Bindings>;

Match<TranslationRule::ExprRule> SearchExpr(const std::string &str);
Match<TranslationRule::TypeRule> SearchType(const std::string &str);
TranslationRule::ExprRule *Search(clang::ASTContext &ctx,
                                  const clang::Expr *expr);
Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                        clang::QualType qual_type);
bool HasExprKey(const std::string &str);

Model CurrentModel();

void Load(Model model, const std::string &rules_dir);
void AddRuleForUserDefinedType(clang::ASTContext &ctx, clang::NamedDecl *decl);
} // namespace cpp2rust::RuleRegistry
