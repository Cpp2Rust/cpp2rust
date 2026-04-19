// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <utility>

template <typename T1, typename T2> void t1(std::pair<T1, T2>);

template <typename T1, typename T2> T2 &f1(std::pair<T1, T2> &o) {
  return o.second;
}

template <typename T1, typename T2>
std::pair<T1, T2> f2(const std::pair<T1, T2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2>
std::pair<T1, T2> f4(const T1 &a0, const T2 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <class T1, class T2, class T3, class T4>
std::pair<T1, T2> f5(const T3 &a0, T4 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <class T1, class T2, class T3, class T4>
std::pair<T1, T2> f6(T3 &a0, T4 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f7(T3 &&a0, T4 &&a1) {
  return std::pair<T1, T2>(std::move(a0), std::move(a1));
}

template <class T1, class T2> auto f9(T1 &&a0, T2 &a1) {
  return std::make_pair(std::move(a0), a1);
}

template <class T1, class T2> auto f10(T1 &&a0, T2 &&a1) {
  return std::make_pair(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 &f11(std::pair<T1, T2> &a0) {
  return a0.first;
}
