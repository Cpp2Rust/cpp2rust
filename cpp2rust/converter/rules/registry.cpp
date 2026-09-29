// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/registry.h"

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <memory>
#include <utility>

#include "converter/converter_lib.h"
#include "converter/printer.h"
#include "converter/rules/string_matcher.h"

namespace cpp2rust::RuleRegistry {

namespace {

Model model_ = Model::kUnsafe;
bool translation_rules_loaded_ = false;

std::unique_ptr<Matching::Matcher> matcher_;

ExprRuleMap exprs_; // key -> ExprRule
TypeRuleMap types_; // key -> TypeRule

void AddTypeRule(std::string src, TranslationRule::TypeRule &&rule) {
  rule.src = std::move(src);
  auto key = matcher_->Key(rule);
  types_.emplace(std::move(key), std::move(rule));
}

void addRulesFromDirectory(const std::filesystem::path &dir, Model model) {
  namespace fs = std::filesystem;
  for (const auto &entry : fs::directory_iterator(dir)) {
    const auto &path = entry.path();
    assert(fs::exists(path / "ir_src.json") &&
           (fs::exists(path / "ir_unsafe.json") ||
            fs::exists(path / "ir_refcount.json")));
    auto [expr_rules, type_rules] = TranslationRule::Load(path, model);
    if (expr_rules.empty() && type_rules.empty()) {
      log() << "No rules found in " << path << '\n';
      continue;
    }
    for (auto &[_, rule] : expr_rules) {
      exprs_.emplace(matcher_->Key(rule), std::move(rule));
    }
    for (auto &[_, rule] : type_rules) {
      auto key = matcher_->Key(rule);
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

Matching::Match<TranslationRule::ExprRule> Search(clang::ASTContext &ctx,
                                                  const clang::Expr *expr) {
  if (RefersToUserDefinedDecl(expr)) {
    return {};
  }
  return matcher_->Find(ctx, expr);
}

Matching::Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                                  clang::QualType qual_type) {
  return matcher_->Find(ctx, qual_type);
}

Matching::Matcher &GetMatcher() { return *matcher_; }

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

  matcher_ = std::make_unique<Matching::StringMatcher>();
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
