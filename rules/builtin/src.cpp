// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

bool CPP2RUST_EXPR_RULE(9)(long a, long b, long *r) {
  return __builtin_mul_overflow(a, b, r);
}
bool CPP2RUST_EXPR_RULE(10)(long long a, long long b, long long *r) {
  return __builtin_mul_overflow(a, b, r);
}
