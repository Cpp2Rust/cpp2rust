#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstdint>
#include <deque>
#include <string>

namespace cpp2rust {

struct RsExpr {
  enum class Kind : uint8_t {
    Verbatim,
  };

  Kind kind;
  std::string text;
};

class RsArena {
public:
  RsExpr *Verbatim(std::string text);

private:
  std::deque<RsExpr> pool_;
};

std::string Print(const RsExpr *expr);

} // namespace cpp2rust
