// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <utility>
#endif

template <typename T1, typename T2>
using CPP2RUST_TYPE_RULE(1) = std::pair<T1, T2>;

template <typename T1, typename T2>
T2 &CPP2RUST_EXPR_RULE(1)(std::pair<T1, T2> &o) {
  return o.second;
}

template <typename T1, typename T2>
std::pair<T1, T2> CPP2RUST_EXPR_RULE(2)(const std::pair<T1, T2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2>
std::pair<T1, T2> CPP2RUST_EXPR_RULE(4)(const T1 &a0, const T2 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <class T1, class T2, class T3, class T4>
std::pair<T1, T2> CPP2RUST_EXPR_RULE(5)(const T3 &a0, T4 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <class T1, class T2, class T3, class T4>
std::pair<T1, T2> CPP2RUST_EXPR_RULE(6)(T3 &a0, T4 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> CPP2RUST_EXPR_RULE(7)(T3 &&a0, T4 &&a1) {
  return std::pair<T1, T2>(std::move(a0), std::move(a1));
}

template <class T1, class T2> auto CPP2RUST_EXPR_RULE(9)(T1 &&a0, T2 &a1) {
  return std::make_pair(std::move(a0), a1);
}

template <class T1, class T2> auto CPP2RUST_EXPR_RULE(10)(T1 &&a0, T2 &&a1) {
  return std::make_pair(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 &CPP2RUST_EXPR_RULE(11)(std::pair<T1, T2> &a0) {
  return a0.first;
}

template <typename T1, typename T2>
std::pair<T1, T2> CPP2RUST_EXPR_RULE(12)(std::pair<T1, T2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::pair<T1, T2> &CPP2RUST_EXPR_RULE(13)(std::pair<T1, T2> &dst,
                                          const std::pair<T1, T2> &src) {
  return dst.operator=(src);
}

template <typename T1, typename T2>
std::pair<T1, T2> &CPP2RUST_EXPR_RULE(14)(std::pair<T1, T2> &dst,
                                          std::pair<T1, T2> &&src) {
  return dst.operator=(std::move(src));
}
