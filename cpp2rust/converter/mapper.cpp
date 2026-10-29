// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/ExprCXX.h>

#include <cstdlib>
#include <format>
#include <optional>
#include <utility>
#include <vector>

#include "converter/converter.h"
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

bool IsRefExpr(clang::ASTContext &ctx, const clang::Expr *expr) {
  const auto *rule = RuleRegistry::GetExprRule(ctx, expr);
  return rule && rule->refExprOperand();
}

std::string MapFunctionName(clang::ASTContext &ctx,
                            const clang::FunctionDecl *decl) {
  assert(decl);
  if (!IsUserDefinedDecl(decl) && RuleRegistry::HasExprRule(decl)) {
    return std::format("libcc2rs::{}_{}", decl->getNameAsString(),
                       RuleRegistry::CurrentModel() == Model::kRefCount
                           ? "refcount"
                           : "unsafe");
  }
  return GetNamedDeclAsString(decl->getCanonicalDecl());
}

std::string InstantiateTemplate(Converter &converter, const clang::Expr *expr,
                                unsigned n) {
  auto [rule, subs] = RuleRegistry::Search(converter.GetASTContext(), expr);
  if (!rule) {
    return std::format("T{}", n);
  }
  return Matcher::MapBinding(converter, subs, n - 1);
}

std::string InstantiateTemplateDefault(Converter &converter,
                                       const clang::Expr *expr, unsigned n) {
  auto [rule, subs] = RuleRegistry::Search(converter.GetASTContext(), expr);
  assert(rule && "default of a generic outside a matched rule");
  auto &arg = subs.at(n - 1);
  assert(arg.getKind() == clang::TemplateArgument::Type &&
         "default of a non-type generic");
  return converter.GetUnboxedDefaultAsString(arg.getAsType());
}

std::string Map(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = RuleRegistry::Search(ctx, qual_type).first;
  return rule ? rule->type_info.type : std::string();
}

std::string Map(Converter &converter, clang::QualType qual_type) {
  auto [rule, subs] =
      RuleRegistry::Search(converter.GetASTContext(), qual_type);
  if (rule) {
    return Matcher::InstantiateTgt(converter, subs, rule->type_info.type);
  }
  return {};
}

std::string MapInitializer(Converter &converter, clang::QualType qual_type) {
  auto [rule, subs] =
      RuleRegistry::Search(converter.GetASTContext(), qual_type);
  if (rule && !rule->initializer.empty()) {
    return Matcher::InstantiateTgt(converter, subs, rule->initializer);
  }
  return {};
}

std::string GetParamType(Converter &converter, const clang::Expr *expr,
                         unsigned index) {
  auto [rule, subs] = RuleRegistry::Search(converter.GetASTContext(), expr);
  return Matcher::InstantiateTgt(converter, subs, rule->params.at(index).type);
}

std::string GetReturnType(Converter &converter, const clang::Expr *expr) {
  auto [rule, subs] = RuleRegistry::Search(converter.GetASTContext(), expr);
  return Matcher::InstantiateTgt(converter, subs, rule->return_type.type);
}

} // namespace cpp2rust::Mapper
