// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/string_matcher.h"

#include <algorithm>
#include <cassert>
#include <cctype>
#include <string_view>

#include "converter/converter_lib.h"
#include "converter/mapper.h"
#include "converter/printer.h"
#include "converter/rules/registry.h"

namespace cpp2rust::Matching {

namespace {

// Attempts to unify an instantiated C++ type or function signature with a
// corresponding template pattern. If the two match structurally, it returns
// a mapping from template parameter names (e.g., "T1") to their concrete
// instantiated types (e.g., "int"). If no match is possible, returns nullopt.
//
// Example:
//   template_str   = "std::vector<T1>::vector()"
//   instantiated   = "std::vector<int>::vector()"
//   result         = { "int" }
std::optional<Bindings> matchTemplate(const std::string &template_str,
                                      const std::string &instantiated) {
  auto matchLiteralAt = [&](const std::string &input_str, size_t pos,
                            std::string_view literal, size_t &end_pos) -> bool {
    size_t i = pos;
    size_t j = 0;

    while (true) {
      while (i < input_str.size() && std::isspace(input_str[i])) {
        i++;
      }

      while (j < literal.size() && std::isspace(literal[j])) {
        j++;
      }

      if (j == literal.size()) {
        end_pos = i;
        return true;
      }

      if (i >= input_str.size()) {
        return false;
      }

      if (input_str[i] != literal[j]) {
        return false;
      }

      i++;
      j++;
    }
  };

  auto findNextLiteralSameDepth = [&](const std::string &s, size_t start,
                                      std::string_view lit) -> size_t {
    int ang = 0;
    int par = 0;
    int sq = 0;

    for (size_t i = 0; i < s.size() && i < start; i++) {
      switch (s[i]) {
      case '<': {
        ang++;
        break;
      }
      case '>': {
        ang--;
        break;
      }
      case '(': {
        par++;
        break;
      }
      case ')': {
        par--;
        break;
      }
      case '[': {
        sq++;
        break;
      }
      case ']': {
        sq--;
        break;
      }
      default:
        break;
      }
      assert(ang >= 0 && par >= 0 && sq >= 0 && "Unbalanced ang, par or sq");
    }

    int base_ang = ang;
    int base_par = par;
    int base_sq = sq;

    for (size_t i = start; i <= s.size(); i++) {
      if (ang == base_ang && par == base_par && sq == base_sq) {
        size_t end_i = 0;
        if (matchLiteralAt(s, i, lit, end_i)) {
          return i;
        }
      }

      if (i == s.size()) {
        break;
      }

      char c = s[i];
      switch (c) {
      case '<': {
        ang++;
        break;
      }
      case '>': {
        ang--;
        break;
      }
      case '(': {
        par++;
        break;
      }
      case ')': {
        par--;
        break;
      }
      case '[': {
        sq++;
        break;
      }
      case ']': {
        sq--;
        break;
      }
      default:
        break;
      }

      if (ang < 0 || par < 0 || sq < 0) {
        return std::string::npos;
      }
    }

    return std::string::npos;
  };

  Bindings captured;

  size_t ti = 0;
  size_t si = 0;

  while (ti < template_str.size()) {
    if (template_str[ti] == 'T' && ti + 1 < template_str.size() &&
        std::isdigit(template_str[ti + 1])) {
      size_t tj = ti + 2;
      while (tj < template_str.size() && std::isdigit(template_str[tj])) {
        tj++;
      }

      size_t type_idx = std::stoi(&template_str[ti + 1]) - 1;
      assert(type_idx < TranslationRule::kMaxGenerics &&
             "template placeholder exceeds kMaxGenerics");
      ti = tj;

      std::string_view nextLit;
      size_t scan = ti;
      while (scan < template_str.size()) {
        if (template_str[scan] == 'T' && scan + 1 < template_str.size() &&
            std::isdigit(template_str[scan + 1])) {
          break;
        }
        scan++;
      }
      nextLit = std::string_view(template_str).substr(ti, scan - ti);

      captured.resize(std::max(captured.size(), type_idx + 1));
      auto &repl = captured[type_idx];
      if (repl.has_value()) {
        size_t end_pos = 0;
        if (!matchLiteralAt(instantiated, si, *repl, end_pos)) {
          return std::nullopt;
        }
        si = end_pos;
      } else {
        if (!nextLit.empty()) {
          size_t k = findNextLiteralSameDepth(instantiated, si, nextLit);
          if (k == std::string::npos) {
            return std::nullopt;
          }

          size_t a = si;
          size_t b = k;

          while (a < b && std::isspace(instantiated[a])) {
            a++;
          }
          while (b > a && std::isspace(instantiated[b - 1])) {
            b--;
          }

          repl = instantiated.substr(a, b - a);
          si = k;
        } else {
          size_t a = si;
          size_t b = instantiated.size();

          while (a < b && std::isspace(instantiated[a])) {
            a++;
          }
          while (b > a && std::isspace(instantiated[b - 1])) {
            b--;
          }

          repl = instantiated.substr(a, b - a);
          si = instantiated.size();
        }
      }

      if (!nextLit.empty()) {
        size_t end_pos = 0;
        if (!matchLiteralAt(instantiated, si, nextLit, end_pos)) {
          return std::nullopt;
        }
        si = end_pos;
        ti += nextLit.size();
      }
    } else {
      size_t tj = ti;
      while (tj < template_str.size()) {
        if (template_str[tj] == 'T' && tj + 1 < template_str.size() &&
            std::isdigit(template_str[tj + 1])) {
          break;
        }
        ++tj;
      }

      auto lit = std::string_view(template_str).substr(ti, tj - ti);
      size_t end_pos = 0;
      if (!matchLiteralAt(instantiated, si, lit, end_pos)) {
        return std::nullopt;
      }
      si = end_pos;
      ti = tj;
    }
  }

  while (si < instantiated.size() && std::isspace(instantiated[si])) {
    si++;
  }

  if (si != instantiated.size()) {
    return std::nullopt;
  }

  return captured;
}

std::string exprKey(const std::string &str) {
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

std::string typeKey(const std::string &str) {
  auto n = str.find_first_of("<[");
  if (n == std::string::npos || str[n] == '<') {
    return str.substr(0, n);
  }
  // something like int[][] or T1[] -> []
  return str.substr(n + 1);
}

template <typename Rule, typename Candidates>
Match<Rule> search(Candidates candidates, const std::string &txt) {
  Rule *rule = nullptr;
  Bindings subs;

  for (auto &[_, this_rule] : candidates) {
    auto this_subs = matchTemplate(this_rule.src, txt);
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

Match<TranslationRule::ExprRule> searchExpr(const std::string &txt) {
  return search<TranslationRule::ExprRule>(
      RuleRegistry::ExprCandidates(exprKey(txt)), txt);
}

Match<TranslationRule::TypeRule> searchType(const std::string &txt) {
  return search<TranslationRule::TypeRule>(
      RuleRegistry::TypeCandidates(typeKey(txt)), txt);
}

std::string mapTypeString(const std::string &cpp_type) {
  auto [rule, subs] = searchType(cpp_type);
  if (!rule) {
    llvm::errs() << "cpp_type: " << cpp_type << '\n';
    assert(0 && "Type is not present in the registry");
  }
  for (auto &ty : subs) {
    if (ty) {
      ty = mapTypeString(*ty);
    }
  }
  return Mapper::InstantiateTgt(subs, rule->type_info.type);
}

} // namespace

std::string StringMatcher::Key(const TranslationRule::ExprRule &rule) const {
  return exprKey(rule.src);
}

std::string StringMatcher::Key(const TranslationRule::TypeRule &rule) const {
  return typeKey(rule.src);
}

Match<TranslationRule::ExprRule> StringMatcher::Find(clang::ASTContext &ctx,
                                                     const clang::Expr *expr) {
  auto qualified_name = Printer::ToString(ctx, expr);
  auto res = searchExpr(qualified_name);
  log() << "search expr " << qualified_name << ", result:\n";
  if (res.first) {
    res.first->dump();
  } else {
    log() << "None\n";
  }
  return res;
}

Match<TranslationRule::TypeRule> StringMatcher::Find(clang::ASTContext &ctx,
                                                     clang::QualType type) {
  auto sugared = Printer::ToString(ctx, type, Printer::ScalarSugar::kPreserve);
  if (auto res = searchType(sugared); res.first) {
    log() << "search type " << sugared
          << ", result: " << res.first->type_info.type << '\n';
    return res;
  }
  auto desugared = Printer::ToString(ctx, type);
  if (desugared == sugared) {
    log() << "search type " << desugared << ", result: None\n";
    return {};
  }
  auto res = searchType(desugared);
  log() << "search type " << desugared
        << ", result: " << (res.first ? res.first->type_info.type : "None")
        << '\n';
  return res;
}

bool StringMatcher::HasRuleNamed(clang::ASTContext &ctx,
                                 const clang::FunctionDecl *decl) {
  return !RuleRegistry::ExprCandidates(exprKey(Printer::ToString(ctx, decl)))
              .empty();
}

std::string StringMatcher::MapBinding(clang::ASTContext &ctx,
                                      const Bindings &bindings, unsigned n) {
  return mapTypeString(bindings.at(n).value());
}

} // namespace cpp2rust::Matching
