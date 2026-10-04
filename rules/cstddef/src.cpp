// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <cstddef>
#endif

using CPP2RUST_TYPE_RULE(1) = std::byte;

std::byte CPP2RUST_EXPR_RULE(1)(const std::byte &a0, unsigned a1) {
  return operator<<(a0, a1);
}

std::byte CPP2RUST_EXPR_RULE(2)(const std::byte &a0, unsigned a1) {
  return operator>>(a0, a1);
}

std::byte CPP2RUST_EXPR_RULE(3)(std::byte &a0, unsigned a1) {
  return operator<<=(a0, a1);
}

std::byte CPP2RUST_EXPR_RULE(4)(std::byte &a0, unsigned a1) {
  return operator>>=(a0, a1);
}
