// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <new>
#endif

void *CPP2RUST_EXPR_RULE(1)(std::size_t a0) { return ::operator new(a0); }

void CPP2RUST_EXPR_RULE(2)(void *a0) { return ::operator delete(a0); }

void *CPP2RUST_EXPR_RULE(3)(std::size_t a0) { return ::operator new[](a0); }

void CPP2RUST_EXPR_RULE(4)(void *a0) { return ::operator delete[](a0); }

template <typename T1> T1 *CPP2RUST_EXPR_RULE(5)(T1 *a0) {
  return std::launder(a0);
}
