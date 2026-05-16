#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Expr.h>

#include <filesystem>
#include <string>
#include <unordered_map>
#include <variant>
#include <vector>

#include "converter/factory.h"

namespace cpp2rust::TranslationRule {

inline constexpr unsigned kMaxGenerics = 9;

struct TextFragment {
  std::string text;

  void dump() const;
};

enum class Access { kRead, kWrite, kMove };

struct PlaceholderFragment {
  unsigned n; // "a0", "a1", ...
  Access access;

  void dump() const;
};

struct GenericFragment {
  unsigned n; // "T1", "T2", ...

  void dump() const;
};

struct MethodCallFragment; // forward declaration

using BodyFragment =
    std::variant<TextFragment, PlaceholderFragment, GenericFragment,
                 std::unique_ptr<MethodCallFragment>>;

struct MethodCallFragment {
  std::vector<BodyFragment> receiver;
  std::vector<BodyFragment> body;

  const PlaceholderFragment *getReceiverPlaceholder() const;
  void dump() const;
};

struct TypeInfo {
  std::string type;
  bool is_refcount_pointer = false;
  bool is_unsafe_pointer = false;

  bool is_pointer() const { return is_refcount_pointer || is_unsafe_pointer; }

  void dump() const;
};

struct ExprRule {
  std::string src;
  std::vector<TypeInfo> params;
  TypeInfo return_type;
  std::vector<std::vector<std::string>> generics; // "T1" -> ["Ord", "Clone"]
  std::vector<BodyFragment> body;
  bool multi_statement = false;

  void dump() const;
  void validate(const std::string &name) const;
};

struct TypeRule {
  std::string src;
  std::string initializer; // Rust initializer expression
  TypeInfo type_info;

  void dump() const;

  static TypeRule Plain(std::string type) {
    return {{}, {}, {std::move(type), false, false}};
  }
  static TypeRule RefcountPtr(std::string type) {
    return {{}, {}, {std::move(type), true, false}};
  }
  static TypeRule UnsafePtr(std::string type) {
    return {{}, {}, {std::move(type), false, true}};
  }
};

using ExprRules = std::unordered_map<std::string, ExprRule>;
using TypeRules = std::unordered_map<std::string, TypeRule>;

std::pair<ExprRules, TypeRules> Load(const std::filesystem::path &path,
                                     Model model);
} // namespace cpp2rust::TranslationRule
