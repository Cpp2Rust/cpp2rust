// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <cmath>
#endif

int CPP2RUST_EXPR_RULE(1)(int x) { return std::abs(x); }

long CPP2RUST_EXPR_RULE(2)(long x) { return std::abs(x); }

long long CPP2RUST_EXPR_RULE(3)(long long x) { return std::abs(x); }

double CPP2RUST_EXPR_RULE(4)(double x) { return std::log2(x); }

double CPP2RUST_EXPR_RULE(5)(int x) { return std::log2(x); }
