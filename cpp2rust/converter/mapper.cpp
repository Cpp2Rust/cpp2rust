// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/ExprCXX.h>

#include <cstdlib>
#include <format>
#include <optional>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/rules/matcher.h"
#include "converter/rules/registry.h"
#include "converter/translation_rule.h"

namespace cpp2rust::Mapper {

bool Contains(clang::ASTContext &ctx, clang::QualType qual_type) {
  return RuleRegistry::Search(ctx, qual_type).first != nullptr;
}

bool Contains(clang::ASTContext &ctx, const clang::Expr *expr) {
  return RuleRegistry::Search(ctx, expr).first != nullptr;
}

std::string MapFunctionName(clang::ASTContext &ctx,
                            const clang::FunctionDecl *decl) {
  assert(decl);
  if (!IsUserDefinedDecl(decl) && Matcher::HasRuleNamed(ctx, decl)) {
    return std::format("libcc2rs::{}_{}", decl->getNameAsString(),
                       RuleRegistry::CurrentModel() == Model::kRefCount
                           ? "refcount"
                           : "unsafe");
  }
  return GetNamedDeclAsString(decl->getCanonicalDecl());
}

std::string InstantiateTemplate(clang::ASTContext &ctx, const clang::Expr *expr,
                                unsigned n) {
  auto [rule, subs] = RuleRegistry::Search(ctx, expr);
  auto text = std::format("T{}", n);
  if (!rule) {
    return text;
  }
  auto &ty = subs.at(n - 1);
  if (ty) {
    ty = Matcher::MapBinding(subs, n - 1);
  }
  return Matcher::InstantiateTgt(subs, text);
}

std::string Map(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto [rule, subs] = RuleRegistry::Search(ctx, qual_type);
  if (rule) {
    return Matcher::InstantiateTgt(Matcher::MapBindings(subs),
                                   rule->type_info.type);
  }
  return {};
}

std::string MapInitializer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto [rule, subs] = RuleRegistry::Search(ctx, qual_type);
  if (rule && !rule->initializer.empty()) {
    return Matcher::InstantiateTgt(Matcher::MapBindings(subs),
                                   rule->initializer);
  }
  return {};
}

std::string GetParamType(clang::ASTContext &ctx, const clang::Expr *expr,
                         unsigned index) {
  auto [rule, subs] = RuleRegistry::Search(ctx, expr);
  return Matcher::InstantiateTgt(Matcher::MapBindings(subs),
                                 rule->params.at(index).type);
}

} // namespace cpp2rust::Mapper
