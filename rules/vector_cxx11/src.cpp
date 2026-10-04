// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <vector>
#endif

template <typename T, typename A> using Init = A;

template <typename T1, typename... Args>
void CPP2RUST_EXPR_RULE(1)(std::vector<T1> &o, Init<T1, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}

template <typename T1, typename... Args>
void CPP2RUST_EXPR_RULE(2)(std::vector<std::vector<T1>> &o,
                           Init<std::vector<T1>, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}

template <typename T1, typename T2 = std::allocator<T1>, typename... Args>
void CPP2RUST_EXPR_RULE(3)(std::vector<T1, T2> &o, Init<T1, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}
