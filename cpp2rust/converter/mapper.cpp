// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/ExprCXX.h>
#include <clang/Basic/SourceManager.h>

#include <cctype>
#include <cstdlib>
#include <format>
#include <optional>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/printer.h"
#include "converter/rules/registry.h"
#include "converter/translation_rule.h"

namespace cpp2rust::Mapper {

namespace {

// Substitutes concrete types into a target template string using the provided
// type mapping. Each template parameter in `tgt_template` is replaced with its
// corresponding instantiated type from `types`.
//
// Example:
//   types        = { {"i32"} }
//   tgt_template = "Vec<T1>"
//   result       = "Vec<i32>"
std::string instantiateTgt(const Matching::Bindings &types,
                           const std::string &tgt_template) {
  assert(types.size() <= TranslationRule::kMaxGenerics &&
         "template placeholder exceeds kMaxGenerics");
  std::string instantiated_template = tgt_template;
  std::string::size_type pos = 0;
  while ((pos = instantiated_template.find('T', pos)) != std::string::npos) {
    if (pos + 1 >= instantiated_template.size()) {
      break;
    }
    if (!std::isdigit(instantiated_template[pos + 1])) {
      ++pos;
      continue;
    }
    const auto &repl = types.at(instantiated_template[pos + 1] - '1').value();
    instantiated_template.replace(pos, 2, repl);
    pos += repl.length();
  }
  return instantiated_template;
}

std::string mapTypeStringRecursive(const std::string &cpp_type) {
  auto [rule, subs] = RuleRegistry::SearchType(cpp_type);
  if (!rule) {
    llvm::errs() << "cpp_type: " << cpp_type << '\n';
    assert(0 && "Type is not present in the registry");
  }
  for (auto &ty : subs) {
    if (ty) {
      ty = mapTypeStringRecursive(*ty);
    }
  }
  return instantiateTgt(subs, rule->type_info.type);
}

} // namespace

bool Contains(clang::ASTContext &ctx, clang::QualType qual_type) {
  return RuleRegistry::Search(ctx, qual_type).first != nullptr;
}

bool Contains(clang::ASTContext &ctx, const clang::Expr *expr) {
  return RuleRegistry::Search(ctx, expr) != nullptr;
}

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr) {
  return RuleRegistry::Search(ctx, expr);
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
  if (!IsUserDefinedDecl(decl) &&
      RuleRegistry::HasExprKey(Printer::ToString(ctx, decl))) {
    return std::format("libcc2rs::{}_{}", decl->getNameAsString(),
                       RuleRegistry::CurrentModel() == Model::kRefCount
                           ? "refcount"
                           : "unsafe");
  }
  return GetNamedDeclAsString(decl->getCanonicalDecl());
}

std::string InstantiateTemplate(clang::ASTContext &ctx, const clang::Expr *expr,
                                unsigned n) {
  auto expr_str = Printer::ToString(ctx, expr);
  auto [rule, subs] = RuleRegistry::SearchExpr(expr_str);
  auto text = std::format("T{}", n);
  if (!rule) {
    return text;
  }
  auto &ty = subs.at(n - 1);
  if (ty) {
    ty = mapTypeStringRecursive(*ty);
  }
  return instantiateTgt(subs, text);
}

std::string Map(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto [rule, subs] = RuleRegistry::Search(ctx, qual_type);
  if (rule) {
    for (auto &ty : subs) {
      if (ty) {
        ty = mapTypeStringRecursive(*ty);
      }
    }
    return instantiateTgt(subs, rule->type_info.type);
  }
  return {};
}

std::string MapInitializer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto [rule, subs] = RuleRegistry::Search(ctx, qual_type);
  if (rule && !rule->initializer.empty()) {
    for (auto &ty : subs) {
      if (ty) {
        ty = mapTypeStringRecursive(*ty);
      }
    }
    return instantiateTgt(subs, rule->initializer);
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
  auto rule = RuleRegistry::Search(ctx, expr);
  return rule && rule->return_type.is_pointer();
}

const TranslationRule::TypeInfo &
GetParamInfo(clang::ASTContext &ctx, const clang::Expr *expr, unsigned index) {
  auto rule = RuleRegistry::Search(ctx, expr);
  assert(rule && "expression must have a translation rule");
  return rule->params.at(index);
}

std::string GetParamType(clang::ASTContext &ctx, const clang::Expr *expr,
                         unsigned index) {
  auto expr_str = Printer::ToString(ctx, expr);
  auto [rule, subs] = RuleRegistry::SearchExpr(expr_str);
  for (auto &ty : subs) {
    if (ty) {
      ty = mapTypeStringRecursive(*ty);
    }
  }
  return instantiateTgt(subs, rule->params.at(index).type);
}

bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index) {
  return GetParamInfo(ctx, expr, index).is_pointer();
}

} // namespace cpp2rust::Mapper
