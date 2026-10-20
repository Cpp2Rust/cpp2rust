// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <utility>

template <typename T1, typename T2> using t1 = std::pair<T1, T2>;

template <typename T1, typename T2>
std::pair<T1, T2> f2(const std::pair<T1, T2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f7(T3 &&a0, T4 &&a1) {
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <class T1, class T2>
auto f9(T1 &&a0, T2 &a1) -> decltype(std::make_pair(std::move(a0), a1)) {
  return std::make_pair(std::move(a0), a1);
}

template <class T1, class T2>
auto f10(T1 &&a0, T2 &&a1)
    -> decltype(std::make_pair(std::move(a0), std::move(a1))) {
  return std::make_pair(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::pair<T1, T2> f12(std::pair<T1, T2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::pair<T1, T2> &f13(std::pair<T1, T2> &dst, const std::pair<T1, T2> &src) {
  return dst = src;
}

template <typename T1, typename T2>
std::pair<T1, T2> &f14(std::pair<T1, T2> &dst, std::pair<T1, T2> &&src) {
  return dst = std::move(src);
}

template <typename T1, typename T2> const T2 &f15(const std::pair<T1, T2> &a0) {
  return a0.second;
}

template <typename T1, typename T2> const T1 &f16(const std::pair<T1, T2> &a0) {
  return a0.first;
}

template <typename T1, typename T2> T1 &f17(std::pair<T1, T2> &a0) {
  return std::get<0>(a0);
}

template <typename T1, typename T2> T2 &f18(std::pair<T1, T2> &a0) {
  return std::get<1>(a0);
}

template <typename T1, typename T2> const T1 &f19(const std::pair<T1, T2> &a0) {
  return std::get<0>(a0);
}

template <typename T1, typename T2> const T2 &f20(const std::pair<T1, T2> &a0) {
  return std::get<1>(a0);
}

template <typename T1, typename T2> T1 &&f21(std::pair<T1, T2> &&a0) {
  return std::get<0>(std::move(a0));
}

template <typename T1, typename T2> T2 &&f22(std::pair<T1, T2> &&a0) {
  return std::get<1>(std::move(a0));
}
