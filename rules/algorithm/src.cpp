// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <fstream>
#include <iterator>
#include <string>
#include <vector>

template <typename T1> void f1(T1 first, T1 last) {
  return std::sort(first, last);
}

template <typename T1, typename T2> T2 f2(T1 first, T1 last, T2 d_first) {
  return std::copy(first, last, d_first);
}

template <class T1, class T2> T1 f3(T1 first, T1 last, const T2 &value) {
  return std::find(first, last, value);
}

template <typename T1, typename T2>
void f6(T1 first, T1 last, T2 comp) {
  return std::stable_sort(first, last, comp);
}

template <typename T1> T1 *f8(T1 *first, T1 *last) {
  return std::max_element(first, last);
}

template <typename T1> void f9(T1 &a0, T1 &a1) { return std::swap(a0, a1); }

template <typename T1>
typename std::vector<T1>::iterator f10(typename std::vector<T1>::iterator a0,
                                       typename std::vector<T1>::iterator a1) {
  return std::unique(a0, a1);
}

template <typename T1, typename T2>
void f12(typename std::vector<T1>::iterator a0,
         typename std::vector<T1>::iterator a1, const T2 &a2) {
  return std::fill(a0, a1, a2);
}

std::ostream_iterator<char> f13(std::string::iterator a0,
                                std::string::iterator a1,
                                std::ostream_iterator<char> a2) {
  return std::copy(a0, a1, a2);
}

template <typename T1, typename T2>
void f14(T1 *first, T1 *last, T2 comp) {
  return std::stable_sort(first, last, comp);
}

template <typename T1> const T1 &f16(const T1 &a, const T1 &b) {
  return std::min(a, b);
}

template <typename T1> const T1 &f17(const T1 &a, const T1 &b) {
  return std::max(a, b);
}
