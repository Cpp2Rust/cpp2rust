// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <functional>

template <typename T1> using t1 = std::reference_wrapper<T1>;

template <typename T1> std::reference_wrapper<T1> f1(T1 &a0) {
  return std::reference_wrapper<T1>(a0);
}

template <typename T1> T1 &f2(const std::reference_wrapper<T1> &a0) {
  return a0.operator T1 &();
}

template <typename T1> T1 &f3(const std::reference_wrapper<T1> &a0) {
  return a0.get();
}
