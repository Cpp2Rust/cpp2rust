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

namespace cpp2rust::Matching {
using Bindings = std::vector<std::optional<std::string>>;

template <typename Rule> using Match = std::pair<Rule *, Bindings>;

class Matcher {
public:
  virtual ~Matcher() = default;

  virtual std::string Key(const TranslationRule::ExprRule &rule) const = 0;
  virtual std::string Key(const TranslationRule::TypeRule &rule) const = 0;

  virtual Match<TranslationRule::ExprRule> Find(clang::ASTContext &ctx,
                                                const clang::Expr *expr) = 0;
  virtual Match<TranslationRule::TypeRule> Find(clang::ASTContext &ctx,
                                                clang::QualType type) = 0;

  virtual bool HasRuleNamed(clang::ASTContext &ctx,
                            const clang::FunctionDecl *decl) = 0;

  virtual std::string MapBinding(clang::ASTContext &ctx,
                                 const Bindings &bindings, unsigned n) = 0;
};

std::string InstantiateTgt(const Bindings &types,
                           const std::string &tgt_template);
} // namespace cpp2rust::Matching
