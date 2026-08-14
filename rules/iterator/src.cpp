// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>

#include "rule_hints.h"

template <typename T1 = Iterator<Plain, Long>>
using t1 = std::reverse_iterator<T1>;

template <typename T1 = Iterator<Plain, Long>>
using t2 = typename std::reverse_iterator<T1>::difference_type;

template <typename T1 = Iterator<Plain, Long>> std::reverse_iterator<T1> f1() {
  return std::reverse_iterator<T1>();
}

template <typename T1 = Iterator<Plain, Long>>
std::reverse_iterator<T1>
f2(typename std::reverse_iterator<T1>::iterator_type a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1 = Iterator<Plain, Long>>
typename std::reverse_iterator<T1>::iterator_type
f3(const std::reverse_iterator<T1> &a0) {
  return a0.base();
}

template <typename T2, typename T1 = Iterator<T2, Long>>
typename std::reverse_iterator<T1>::reference
f4(const std::reverse_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T2, typename T1 = Iterator<T2, Long>>
typename std::reverse_iterator<T1>::pointer
f5(const std::reverse_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1 = Iterator<Plain, Long>>
std::reverse_iterator<T1> &f6(std::reverse_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1 = Iterator<Plain, Long>>
std::reverse_iterator<T1> f7(std::reverse_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T2 = Long, typename T1 = Iterator<Plain, T2>>
std::reverse_iterator<T1>
f8(const std::reverse_iterator<T1> &a0,
   typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator+(a1);
}

template <typename T2 = Long, typename T1 = Iterator<Plain, T2>>
std::reverse_iterator<T1>
f9(const std::reverse_iterator<T1> &a0,
   typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator-(a1);
}

template <typename T1 = Iterator<Plain, Long>,
          typename T2 = Iterator<ImplicitlyConvertible, Long>>
bool f10(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return operator==(a0, a1);
}
