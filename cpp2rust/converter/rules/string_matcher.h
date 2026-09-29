#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/matching.h"

namespace cpp2rust::Matching {
class StringMatcher final : public Matcher {
public:
  std::string Key(const TranslationRule::ExprRule &rule) const override;
  std::string Key(const TranslationRule::TypeRule &rule) const override;
  Match<TranslationRule::ExprRule> Find(clang::ASTContext &ctx,
                                        const clang::Expr *expr) override;
  Match<TranslationRule::TypeRule> Find(clang::ASTContext &ctx,
                                        clang::QualType type) override;
  bool HasRuleNamed(clang::ASTContext &ctx,
                    const clang::FunctionDecl *decl) override;
  std::string MapBinding(clang::ASTContext &ctx, const Bindings &bindings,
                         unsigned n) override;
};
} // namespace cpp2rust::Matching
