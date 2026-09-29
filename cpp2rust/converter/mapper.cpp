// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/ExprCXX.h>
#include <clang/Basic/SourceManager.h>

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

namespace {

Matcher::Bindings mapBindings(clang::ASTContext &ctx,
                              const Matcher::Bindings &bindings) {
  Matcher::Bindings mapped(bindings.size());
  for (unsigned i = 0; i < bindings.size(); ++i) {
    if (bindings[i]) {
      mapped[i] = Matcher::MapBinding(ctx, bindings, i);
    }
  }
  return mapped;
}

} // namespace

bool Contains(clang::ASTContext &ctx, clang::QualType qual_type) {
  return RuleRegistry::Search(ctx, qual_type).first != nullptr;
}

bool Contains(clang::ASTContext &ctx, const clang::Expr *expr) {
  return RuleRegistry::Search(ctx, expr).first != nullptr;
}

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr) {
  return RuleRegistry::Search(ctx, expr).first;
}

bool IsLibcPassthrough(clang::ASTContext &ctx, const clang::Expr *expr) {
  const auto *tgt_ir = GetExprRule(ctx, expr);
  if (tgt_ir == nullptr || !tgt_ir->body.empty() || !tgt_ir->is_extern) {
    return false;
  }
  const auto *ref =
      clang::dyn_cast<clang::DeclRefExpr>(expr->IgnoreParenImpCasts());
  const auto *decl = ref != nullptr ? ref->getDecl() : nullptr;
  return decl != nullptr &&
         decl->getASTContext().getSourceManager().isInSystemHeader(
             decl->getLocation());
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
    ty = Matcher::MapBinding(ctx, subs, n - 1);
  }
  return Matcher::InstantiateTgt(subs, text);
}

std::string Map(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto [rule, subs] = RuleRegistry::Search(ctx, qual_type);
  if (rule) {
    return Matcher::InstantiateTgt(mapBindings(ctx, subs),
                                   rule->type_info.type);
  }
  return {};
}

std::string MapInitializer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto [rule, subs] = RuleRegistry::Search(ctx, qual_type);
  if (rule && !rule->initializer.empty()) {
    return Matcher::InstantiateTgt(mapBindings(ctx, subs), rule->initializer);
  }
  return {};
}

bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = RuleRegistry::Search(ctx, qual_type).first;
  return rule && rule->type_info.is_pointer();
}

bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = RuleRegistry::Search(ctx, qual_type).first;
  return rule && rule->type_info.is_refcount_pointer;
}

const std::vector<std::string> *MappedDerives(clang::ASTContext &ctx,
                                              clang::QualType qual_type) {
  auto rule = RuleRegistry::Search(ctx, qual_type).first;
  return rule ? &rule->type_info.derives : nullptr;
}

void SetDerives(clang::ASTContext &ctx, clang::QualType qual_type,
                std::vector<std::string> derives) {
  if (auto *rule = RuleRegistry::Search(ctx, qual_type).first) {
    rule->type_info.derives = std::move(derives);
  }
}

bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr) {
  auto rule = RuleRegistry::Search(ctx, expr).first;
  return rule && rule->return_type.is_pointer();
}

const TranslationRule::TypeInfo &
GetParamInfo(clang::ASTContext &ctx, const clang::Expr *expr, unsigned index) {
  auto rule = RuleRegistry::Search(ctx, expr).first;
  assert(rule && "expression must have a translation rule");
  return rule->params.at(index);
}

std::string GetParamType(clang::ASTContext &ctx, const clang::Expr *expr,
                         unsigned index) {
  auto [rule, subs] = RuleRegistry::Search(ctx, expr);
  return Matcher::InstantiateTgt(mapBindings(ctx, subs),
                                 rule->params.at(index).type);
}

bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index) {
  return GetParamInfo(ctx, expr, index).is_pointer();
}

} // namespace cpp2rust::Mapper
