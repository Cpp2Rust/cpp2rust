// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <functional>
#endif

template <typename T1> using CPP2RUST_TYPE_RULE(1) = std::reference_wrapper<T1>;

template <typename T1>
std::reference_wrapper<T1> CPP2RUST_EXPR_RULE(1)(T1 &a0) {
  return std::reference_wrapper<T1>(a0);
}

template <typename T1>
T1 &CPP2RUST_EXPR_RULE(2)(const std::reference_wrapper<T1> &a0) {
  return a0.operator T1 &();
}

template <typename T1>
T1 &CPP2RUST_EXPR_RULE(3)(const std::reference_wrapper<T1> &a0) {
  return a0.get();
}
