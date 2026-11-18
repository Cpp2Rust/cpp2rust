// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <utility>

template <typename T1, typename T2>
bool f28(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f29(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f30(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f31(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f32(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0 >= a1;
}
