// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <limits>
#endif

template <typename T1> T1 CPP2RUST_EXPR_RULE(1)(std::numeric_limits<T1> &a0) {
  return a0.max();
}

template <typename T1> T1 CPP2RUST_EXPR_RULE(2)(std::numeric_limits<T1> &a0) {
  return a0.min();
}
