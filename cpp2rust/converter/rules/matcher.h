#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "converter/translation_rule.h"

namespace cpp2rust::Matcher {
using Bindings = std::vector<std::optional<std::string>>;

template <typename Rule> using Match = std::pair<Rule *, Bindings>;

std::string Key(const TranslationRule::ExprRule &rule);
std::string Key(const TranslationRule::TypeRule &rule);

std::string ExprKey(const std::string &str);

std::string TypeKey(const std::string &str);

Match<TranslationRule::ExprRule> Find(clang::ASTContext &ctx,
                                      const clang::Expr *expr);
Match<TranslationRule::TypeRule> Find(clang::ASTContext &ctx,
                                      clang::QualType type);

bool HasRuleNamed(clang::ASTContext &ctx, const clang::FunctionDecl *decl);

std::string MapBinding(const Bindings &bindings, unsigned n);
Bindings MapBindings(const Bindings &bindings);

std::string InstantiateTgt(const Bindings &types,
                           const std::string &tgt_template);
} // namespace cpp2rust::Matcher
