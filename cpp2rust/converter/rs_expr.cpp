// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rs_expr.h"

namespace cpp2rust {

RsExpr *RsArena::Verbatim(std::string text) {
  pool_.push_back(RsExpr{RsExpr::Kind::Verbatim, std::move(text)});
  return &pool_.back();
}

std::string Print(const RsExpr *expr) { return expr->text; }

} // namespace cpp2rust
