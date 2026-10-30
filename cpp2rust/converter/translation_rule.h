#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Expr.h>
#include <llvm/ADT/STLFunctionalExtras.h>
#include <llvm/Support/JSON.h>

#include <filesystem>
#include <string>
#include <variant>
#include <vector>

#include "converter/factory.h"

namespace cpp2rust::TranslationRule {

static inline constexpr unsigned kMaxGenerics = 9;

struct TextFragment {
  std::string text;

  void dump() const;
};

enum class Access : int8_t { kBorrow, kBorrowMut, kMove, kTake };

struct PlaceholderFragment {
  unsigned n; // "a0", "a1", ...
  Access access;
  bool is_index_base = false;

  void dump() const;
};

struct GenericFragment {
  unsigned n; // "T1", "T2", ...

  void dump() const;
};

struct DefaultFragment {
  unsigned n;

  void dump() const;
};

struct VaArgsFragment {
  void dump() const;
};

struct InitFragment {
  void dump() const;
};

struct MethodCallFragment; // forward declaration
struct RefExprFragment;
struct ConvertFragment;

using BodyFragment = std::variant<
    TextFragment, PlaceholderFragment, GenericFragment, DefaultFragment,
    VaArgsFragment, InitFragment, std::unique_ptr<MethodCallFragment>,
    std::unique_ptr<RefExprFragment>, std::unique_ptr<ConvertFragment>>;

struct MethodCallFragment {
  std::vector<BodyFragment> receiver;
  std::vector<BodyFragment> body;

  const PlaceholderFragment *getReceiverPlaceholder() const;
  void dump() const;
};

struct RefExprFragment {
  std::string prefix;
  std::vector<BodyFragment> inner;

  void dump() const;
};

struct ConvertFragment {
  unsigned to;
  bool is_move;
  std::vector<BodyFragment> src;

  void dump() const;
};

struct Bound {
  std::string trait_name;
  std::vector<unsigned> args;
};

// Whether pred holds for a fragment of body, or for one nested in them.
bool AnyFragment(const std::vector<BodyFragment> &body,
                 llvm::function_ref<bool(const BodyFragment &)> pred);

struct TypeInfo {
  std::vector<std::string> derives;
  std::string type;
  bool is_refcount_pointer = false;
  bool is_unsafe_pointer = false;

  bool is_pointer() const { return is_refcount_pointer || is_unsafe_pointer; }

  void dump() const;
};

struct ExprRule {
  std::vector<TypeInfo> params;
  TypeInfo return_type;
  std::vector<std::vector<Bound>> generics; // "T1" -> [Ord, Clone]
  std::vector<BodyFragment> body;
  bool multi_statement = false;
  bool is_extern = false;

  void dump() const;
  bool usesInit() const;
  const std::vector<BodyFragment> *refExprOperand() const;
};

struct TypeRule {
  std::vector<BodyFragment> initializer; // Rust initializer expression
  unsigned generic = 0;
  TypeInfo type_info;

  void dump() const;
};

ExprRule ParseExprRuleJSON(const llvm::json::Object &obj);

TypeRule ParseTypeRuleJSON(const llvm::json::Object &obj);
} // namespace cpp2rust::TranslationRule
