// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/registry.h"

#include <clang/Basic/SourceManager.h>

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <utility>

#include "converter/converter_lib.h"
#include "converter/printer.h"
#include "rules_loader.h"

namespace cpp2rust::RuleRegistry {

namespace {

Model model_ = Model::kUnsafe;
bool translation_rules_loaded_ = false;

ExprRuleMap exprs_; // key -> ExprRule
TypeRuleMap types_; // key -> TypeRule

void AddTypeRule(std::string src, TranslationRule::TypeRule &&rule) {
  rule.src = std::move(src);
  auto key = Matcher::Key(rule);
  types_.emplace(std::move(key), std::move(rule));
}

void addRulesFromDirectory(const std::filesystem::path &dir, Model model) {
  namespace fs = std::filesystem;
  for (const auto &entry : fs::directory_iterator(dir)) {
    const auto &path = entry.path();
    if (path.filename() == RulesLoader::kIndexDirName) {
      continue;
    }
    assert(fs::exists(path / "ir_src.json") &&
           (fs::exists(path / "ir_unsafe.json") ||
            fs::exists(path / "ir_refcount.json")));
    auto [expr_rules, type_rules] = TranslationRule::Load(path, model);
    if (expr_rules.empty() && type_rules.empty()) {
      log() << "No rules found in " << path << '\n';
      continue;
    }
    for (auto &[_, rule] : expr_rules) {
      exprs_.emplace(Matcher::Key(rule), std::move(rule));
    }
    for (auto &[_, rule] : type_rules) {
      auto key = Matcher::Key(rule);
      auto [begin, end] = types_.equal_range(key);
      for (auto it = begin; it != end; ++it) {
        if (it->second.src == rule.src) {
          llvm::errs() << "ERROR: duplicate type rule for C++ type '"
                       << rule.src << "': maps to both '"
                       << it->second.type_info.type << "' and '"
                       << rule.type_info.type << "'\n";
          std::exit(EXIT_FAILURE);
        }
      }
      types_.emplace(std::move(key), std::move(rule));
    }
  }
}

const TranslationRule::TypeInfo &
GetParamInfo(clang::ASTContext &ctx, const clang::Expr *expr, unsigned index) {
  auto rule = Search(ctx, expr).first;
  assert(rule && "expression must have a translation rule");
  return rule->params.at(index);
}

} // namespace

std::ranges::subrange<ExprRuleMap::iterator>
ExprCandidates(const std::string &key) {
  auto [begin, end] = exprs_.equal_range(key);
  return {begin, end};
}

std::ranges::subrange<TypeRuleMap::iterator>
TypeCandidates(const std::string &key) {
  auto [begin, end] = types_.equal_range(key);
  return {begin, end};
}

Matcher::Match<TranslationRule::ExprRule> Search(clang::ASTContext &ctx,
                                                 const clang::Expr *expr) {
  if (RefersToUserDefinedDecl(expr)) {
    return {};
  }
  return Matcher::Find(ctx, expr);
}

Matcher::Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                                 clang::QualType qual_type) {
  return Matcher::Find(ctx, qual_type);
}

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr) {
  return Search(ctx, expr).first;
}

bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = Search(ctx, qual_type).first;
  return rule && rule->type_info.is_pointer();
}

bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = Search(ctx, qual_type).first;
  return rule && rule->type_info.is_refcount_pointer;
}

const std::vector<std::string> *MappedDerives(clang::ASTContext &ctx,
                                              clang::QualType qual_type) {
  auto rule = Search(ctx, qual_type).first;
  return rule ? &rule->type_info.derives : nullptr;
}

void SetDerives(clang::ASTContext &ctx, clang::QualType qual_type,
                std::vector<std::string> derives) {
  if (auto *rule = Search(ctx, qual_type).first) {
    rule->type_info.derives = std::move(derives);
  }
}

bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr) {
  auto rule = Search(ctx, expr).first;
  return rule && rule->return_type.is_pointer();
}

bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index) {
  return GetParamInfo(ctx, expr, index).is_pointer();
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

Model CurrentModel() { return model_; }

void AddRuleForUserDefinedType(clang::ASTContext &ctx, clang::NamedDecl *decl) {
  auto cpp_name = Printer::ToString(ctx, GetTypeForDecl(ctx, decl));
  auto rs_name = Printer::ToRustName(cpp_name);

  AddTypeRule(cpp_name, TranslationRule::TypeRule::Plain(rs_name));

  if (auto record_decl = llvm::dyn_cast<clang::RecordDecl>(decl)) {
    // Forward declaration
    if (!record_decl->isThisDeclarationADefinition()) {
      return;
    }

    if (auto cxx_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_decl)) {
      if (cxx_decl->isAbstract()) {
        switch (model_) {
        case Model::kUnsafe:
          AddTypeRule(cpp_name + " *", TranslationRule::TypeRule::UnsafePtr(
                                           "*mut dyn " + rs_name));
          break;
        case Model::kRefCount:
          AddTypeRule(cpp_name + " *", TranslationRule::TypeRule::RefcountPtr(
                                           "PtrDyn<dyn " + rs_name + '>'));
          break;
        }
      } else {
        switch (model_) {
        case Model::kUnsafe:
          AddTypeRule(cpp_name + " *",
                      TranslationRule::TypeRule::UnsafePtr("*mut " + rs_name));
          break;
        case Model::kRefCount:
          AddTypeRule(cpp_name + " *", TranslationRule::TypeRule::RefcountPtr(
                                           "Ptr<" + rs_name + '>'));
          break;
        }
      }

      for (auto *nested : GetNestedStructs(cxx_decl)) {
        AddRuleForUserDefinedType(ctx, nested);
      }
    }
  }
}

void Load(Model model, const std::string &rules_dir) {
  model_ = model;

  if (translation_rules_loaded_) {
    return;
  }
  translation_rules_loaded_ = true;

  addRulesFromDirectory(rules_dir, model);

#if 0
  for (auto &[src, rule] : exprs_) {
    log() << "Expr key: " << src << '\n';
    rule.dump();
  }
  for (auto &[src, rule] : types_) {
    log() << "Type key: " << src << '\n';
    rule.dump();
  }
#endif
}

} // namespace cpp2rust::RuleRegistry
