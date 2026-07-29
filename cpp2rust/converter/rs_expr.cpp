// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rs_expr.h"

namespace cpp2rust {

RsExpr *RsArena::Verbatim(std::string text) {
  pool_.push_back(std::make_unique<cpp2rust::Verbatim>(std::move(text)));
  return pool_.back().get();
}

} // namespace cpp2rust
