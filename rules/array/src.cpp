// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <array>
#endif

template <typename T1, std::size_t T2>
using CPP2RUST_TYPE_RULE(1) = std::array<T1, T2>;

template <typename T1, std::size_t T2>
const T1 *CPP2RUST_EXPR_RULE(1)(const std::array<T1, T2> &o) {
  return o.data();
}

template <typename T1, std::size_t T2>
std::size_t CPP2RUST_EXPR_RULE(2)(const std::array<T1, T2> &o) {
  return o.size();
}

template <typename T1, std::size_t T2>
T1 *CPP2RUST_EXPR_RULE(3)(std::array<T1, T2> &o) {
  return o.data();
}

template <typename T1, std::size_t T2>
std::array<T1, T2> CPP2RUST_EXPR_RULE(4)(std::array<T1, T2> &&o) {
  return std::array<T1, T2>(std::move(o));
}

template <typename T1, std::size_t T2>
std::array<T1, T2> &CPP2RUST_EXPR_RULE(5)(std::array<T1, T2> &dst,
                                          std::array<T1, T2> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1, std::size_t T2>
std::array<T1, T2> CPP2RUST_EXPR_RULE(6)(const std::array<T1, T2> &o) {
  return std::array<T1, T2>(o);
}

template <typename T1, std::size_t T2>
std::array<T1, T2> &CPP2RUST_EXPR_RULE(7)(std::array<T1, T2> &dst,
                                          const std::array<T1, T2> &src) {
  return dst.operator=(src);
}
