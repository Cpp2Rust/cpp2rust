#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstdint>
#include <memory>
#include <string>
#include <utility>
#include <vector>

namespace cpp2rust {

struct RsExpr {
  enum class Kind : uint8_t {
    Verbatim,
  };

  explicit RsExpr(Kind kind) : kind(kind) {}
  virtual ~RsExpr() = default;

  virtual std::string print() const = 0;

  Kind kind;
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
