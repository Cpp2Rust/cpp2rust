#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstdint>
#include <memory>
#include <string>
#include <utility>
#include <vector>

namespace clang {
class Expr;
}

namespace cpp2rust {

struct RsExpr {
  enum class Kind : uint8_t {
    Verbatim,
    Concat,
    Delim,
    Unary,
  };

  explicit RsExpr(Kind kind) : kind(kind) {}
  virtual ~RsExpr() = default;

  virtual std::string print() const = 0;

  Kind kind;
  const clang::Expr *expr = nullptr;
};

struct Verbatim : RsExpr {
  explicit Verbatim(std::string text)
      : RsExpr(Kind::Verbatim), text(std::move(text)) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::Verbatim;
  }

  std::string print() const override { return text; }

  std::string text;
};

struct Concat : RsExpr {
  explicit Concat(std::vector<RsExpr *> parts)
      : RsExpr(Kind::Concat), parts(std::move(parts)) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Concat; }

  std::string print() const override {
    std::string result;
    for (auto *part : parts) {
      result += part->print();
      result += ' ';
    }
    return result;
  }

  std::vector<RsExpr *> parts;
};

struct Delim : RsExpr {
  Delim(char open, char close, RsExpr *inner)
      : RsExpr(Kind::Delim), open(open), close(close), inner(inner) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Delim; }

  std::string print() const override {
    std::string result;
    result += open;
    result += ' ';
    result += inner->print();
    result += close;
    result += ' ';
    return result;
  }

  char open;
  char close;
  RsExpr *inner;
};

struct Unary : RsExpr {
  enum class Op : uint8_t {
    Deref,
    Not,
    Neg,
  };

  Unary(Op op, RsExpr *operand)
      : RsExpr(Kind::Unary), op(op), operand(operand) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Unary; }

  std::string print() const override {
    switch (op) {
    case Op::Deref:
      return "*" + operand->print();
    case Op::Not:
      return "!" + operand->print();
    case Op::Neg:
      return "-" + operand->print();
    }
  }

  Op op;
  RsExpr *operand;
};

inline bool SameRendered(const RsExpr *lhs, const RsExpr *rhs) {
  return lhs->print() == rhs->print();
}

class RsArena {
public:
  template <typename T, typename... Args> T *New(Args &&...args) {
    pool_.push_back(std::make_unique<T>(std::forward<Args>(args)...));
    return static_cast<T *>(pool_.back().get());
  }

private:
  std::vector<std::unique_ptr<RsExpr>> pool_;
};

} // namespace cpp2rust
