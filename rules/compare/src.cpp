// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <compare>
#endif

typedef std::strong_ordering CPP2RUST_TYPE_RULE(1);

const std::strong_ordering &CPP2RUST_EXPR_RULE(1)() {
  return std::strong_ordering::less;
}

const std::strong_ordering &CPP2RUST_EXPR_RULE(2)() {
  return std::strong_ordering::equal;
}

const std::strong_ordering &CPP2RUST_EXPR_RULE(3)() {
  return std::strong_ordering::equivalent;
}

const std::strong_ordering &CPP2RUST_EXPR_RULE(4)() {
  return std::strong_ordering::greater;
}

bool CPP2RUST_EXPR_RULE(5)(std::strong_ordering a0, std::strong_ordering a1) {
  return operator==(a0, a1);
}

bool CPP2RUST_EXPR_RULE(6)(std::strong_ordering a0) {
  return operator==(a0, 0);
}

bool CPP2RUST_EXPR_RULE(7)(std::strong_ordering a0) { return operator<(a0, 0); }

bool CPP2RUST_EXPR_RULE(8)(std::strong_ordering a0) { return operator>(a0, 0); }

bool CPP2RUST_EXPR_RULE(9)(std::strong_ordering a0) {
  return operator<=(a0, 0);
}

bool CPP2RUST_EXPR_RULE(10)(std::strong_ordering a0) {
  return operator>=(a0, 0);
}
