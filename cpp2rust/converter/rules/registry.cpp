// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/registry.h"

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <unordered_map>
#include <utility>

#include "converter/converter_lib.h"
#include "converter/printer.h"

namespace cpp2rust::RuleRegistry {

namespace {

using ExprRuleMap =
    std::unordered_multimap<std::string, TranslationRule::ExprRule>;
using TypeRuleMap =
    std::unordered_multimap<std::string, TranslationRule::TypeRule>;

Model model_ = Model::kUnsafe;
bool translation_rules_loaded_ = false;

ExprRuleMap exprs_; // key -> ExprRule
TypeRuleMap types_; // key -> TypeRule

std::string ExprKey(const std::string &str) {
  // Extract the function name from something like
  // const T1 & std::foo<T1, T2>::fn_name(args)
  auto n = str.find_first_of('(');
  if (n == std::string::npos) {
    n = str.size();
  }

  // Walk backwards from '(' tracking <> depth:
  // - skip characters inside template arguments (depth > 0)
  // - stop at the first space outside all angle brackets
  std::string result;
  int depth = 0;
  for (int i = (int)n - 1; i >= 0; --i) {
    char c = str[i];
    if (c == '>')
      ++depth;
    else if (c == '<')
      --depth;
    else if (c == ' ' && depth == 0)
      break;
    else if (depth == 0)
      result += c;
  }
  std::reverse(result.begin(), result.end());
  return result;
}

std::string TypeKey(const std::string &str) {
  auto n = str.find_first_of("<[");
  if (n == std::string::npos || str[n] == '<') {
    return str.substr(0, n);
  }
  // something like int[][] or T1[] -> []
  return str.substr(n + 1);
}

void AddTypeRule(std::string src, TranslationRule::TypeRule &&rule) {
  auto key = TypeKey(src);
  rule.src = std::move(src);
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
      exprs_.emplace(ExprKey(rule.src), std::move(rule));
    }
    for (auto &[_, rule] : type_rules) {
      auto key = TypeKey(rule.src);
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

template <typename T>
Match<T> search(std::unordered_multimap<std::string, T> &map,
                const std::string &txt, const std::string &key) {
  auto [it, end] = map.equal_range(key);
  T *rule = nullptr;
  Matching::Bindings subs;

  for (; it != end; ++it) {
    auto &this_rule = it->second;
    auto this_subs = Matching::MatchTemplate(this_rule.src, txt);
    if (!this_subs) {
      continue;
    }
    // tie breaker: prefer more specific rules (usually the longer ones)
    if (!rule || this_rule.src.size() > rule->src.size()) {
      rule = &this_rule;
      subs = *std::move(this_subs);
    }
  }
  return {rule, std::move(subs)};
}

} // namespace

Match<TranslationRule::ExprRule> SearchExpr(const std::string &str) {
  return search(exprs_, str, ExprKey(str));
}

Match<TranslationRule::TypeRule> SearchType(const std::string &str) {
  return search(types_, str, TypeKey(str));
}

TranslationRule::ExprRule *Search(clang::ASTContext &ctx,
                                  const clang::Expr *expr) {
  if (RefersToUserDefinedDecl(expr)) {
    return nullptr;
  }
  auto qualified_name = Printer::ToString(ctx, expr);
  auto [rule, subs] = SearchExpr(qualified_name);
  log() << "search expr " << qualified_name << ", result:\n";
  if (rule) {
    rule->dump();
  } else {
    log() << "None\n";
  }
  return rule;
}

Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                        clang::QualType qual_type) {
  auto sugared =
      Printer::ToString(ctx, qual_type, Printer::ScalarSugar::kPreserve);
  if (auto res = SearchType(sugared); res.first) {
    log() << "search type " << sugared
          << ", result: " << res.first->type_info.type << '\n';
    return res;
  }
  auto type = Printer::ToString(ctx, qual_type);
  if (type == sugared) {
    log() << "search type " << type << ", result: None\n";
    return {};
  }
  auto res = SearchType(type);
  log() << "search type " << type
        << ", result: " << (res.first ? res.first->type_info.type : "None")
        << '\n';
  return res;
}

bool HasExprKey(const std::string &str) {
  return exprs_.contains(ExprKey(str));
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
