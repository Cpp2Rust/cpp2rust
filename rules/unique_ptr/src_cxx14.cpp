// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> std::unique_ptr<T1[]> f1(std::size_t n) {
  return std::make_unique<T1[]>(n);
}

template <typename T1, typename... T2> std::unique_ptr<T1> f8(T2 &&...a0) {
  return std::make_unique<T1>(std::forward<T2>(a0)...);
}
