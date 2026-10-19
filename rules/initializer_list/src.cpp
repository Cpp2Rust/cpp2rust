// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <initializer_list>

template <typename T1> using t1 = std::initializer_list<T1>;

template <typename T1> using t2 = typename std::initializer_list<T1>::size_type;

template <typename T1>
typename std::initializer_list<T1>::size_type
f1(const std::initializer_list<T1> &o) {
  return o.size();
}

template <typename T1, std::size_t T2>
std::initializer_list<T1> f2(const T1 (&a0)[T2]) {
  return std::initializer_list<T1>{a0};
}
