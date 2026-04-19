// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <array>

template <typename T1, std::size_t T2> void t1(std::array<T1, T2>);

template <typename T1, std::size_t T2>
const T1 *f1(const std::array<T1, T2> &o) {
  return o.data();
}

template <typename T1, std::size_t T2>
std::size_t f2(const std::array<T1, T2> &o) {
  return o.size();
}

template <typename T1, std::size_t T2> T1 *f3(std::array<T1, T2> &o) {
  return o.data();
}
