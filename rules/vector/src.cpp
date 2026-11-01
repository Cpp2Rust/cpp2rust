// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <type_traits>
#include <vector>

template <typename T1, typename T2 = std::allocator<T1>>
using t5 = std::vector<T1, T2>;

template <typename T1, typename T2 = std::allocator<T1>>
using t9 = typename std::vector<T1, T2>::size_type;

template <typename T1, typename T2 = std::allocator<T1>>
using t10 = typename std::vector<T1, T2>::value_type;

template <typename T1>
using t11 = typename std::vector<std::vector<T1>>::value_type;

template <typename T1> using t6 = typename std::vector<bool, T1>::reference;

template <typename T1, typename T2 = std::allocator<T1>>
using t13 = typename std::vector<T1, T2>::reference;

template <typename T1, typename T2 = std::allocator<T1>>
using t14 = typename std::vector<T1, T2>::const_reference;

template <typename T1>
using t15 = typename std::vector<bool, T1>::iterator::reference;

template <typename T1>
using t16 = typename std::vector<bool, T1>::const_iterator::reference;

std::vector<bool> f38(std::size_t n, const bool &value) {
  return std::vector<bool>(n, value);
}

template <class T1, std::size_t T2> const T1 *f40(T1 const (&a0)[T2]) {
  return std::end(a0);
}

bool f47(const std::vector<bool>::reference &o) { return o; }

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f60(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator it) {
  return o.erase(it);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::size_t f61(const std::vector<T1, T2> &o) {
  return o.size();
}

template <typename T1, typename T2 = std::allocator<T1>>
bool f62(const std::vector<T1, T2> &o) {
  return o.empty();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f63() {
  return std::vector<T1, T2>();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f64(std::vector<T1, T2> &o) {
  return o.pop_back();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 *f65(std::vector<T1, T2> &o) {
  return o.data();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &f66(std::vector<T1, T2> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f67(std::size_t n) {
  return std::vector<T1, T2>(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &f68(std::vector<T1, T2> &o) {
  return o.front();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &f69(std::vector<T1, T2> &o) {
  return o.back();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::size_t f70(const std::vector<T1, T2> &o) {
  return o.capacity();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f71(std::vector<T1, T2> &o, std::size_t n) {
  return o.reserve(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator f72(std::vector<T1, T2> &o) {
  return o.begin();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f73(std::vector<T1, T2> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1, typename T2 = std::allocator<T1>>
void f74(std::vector<T1, T2> &o, std::size_t n) {
  return o.resize(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
void f75(std::vector<T1, T2> &o) {
  return o.clear();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator f76(std::vector<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f77(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator it,
    T1 &&value) {
  return o.insert(it, std::move(value));
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f78(std::size_t n, const T1 &value) {
  return std::vector<T1, T2>(n, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f79(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator it,
    const T1 &value) {
  return o.insert(it, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
void f80(std::vector<T1, T2> &o, const T1 &value) {
  return o.push_back(value);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f91(const std::initializer_list<T1> &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2 = std::allocator<T1>, typename T3>
std::vector<T1, T2> f92(T3 first, T3 last) {
  return std::vector<T1, T2>(first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 *f93(const std::vector<T1, T2> &o) {
  return o.data();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator f95(const std::vector<T1, T2> &o) {
  return o.begin();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator f96(const std::vector<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f97(std::vector<T1, T2> &o, std::vector<T1, T2> &a0) {
  return o.swap(a0);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 &f98(const std::vector<T1, T2> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 &f99(const std::vector<T1, T2> &o) {
  return o.back();
}

template <typename T1, typename T2 = std::allocator<T1>, typename T3>
typename std::vector<T1, T2>::iterator
f101(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator pos,
     T3 first, T3 last) {
  return o.insert(pos, first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
void f102(std::vector<T1, T2> &o, std::size_t n,
          const typename std::vector<T1, T2>::value_type &value) {
  return o.resize(n, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> &f103(std::vector<T1, T2> &dst, std::vector<T1, T2> &&src) {
  return dst = std::move(src);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator
f104(const std::vector<T1, T2> &o) {
  return o.cend();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> &f105(std::vector<T1, T2> &dst,
                          const std::vector<T1, T2> &src) {
  return dst = src;
}

template <typename T1, typename T2 = std::allocator<T1>>
void f106(std::vector<T1, T2> &o) {
  return o.shrink_to_fit();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f108(std::vector<T1, T2> &&o) {
  return std::vector<T1, T2>(std::move(o));
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f110(const std::vector<T1, T2> &o) {
  return std::vector<T1, T2>(o);
}

template <typename T1, typename T2 = std::allocator<T1>, typename... T3>
auto f114(std::vector<T1, T2> &o, T3 &&...a1)
    -> decltype(o.emplace_back(std::forward<T3>(a1)...)) {
  return o.emplace_back(std::forward<T3>(a1)...);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f115(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator pos,
     std::size_t n, const T1 &value) {
  return o.insert(pos, n, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f116(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator pos,
     std::initializer_list<T1> l) {
  return o.insert(pos, l);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f117(const typename std::vector<bool, T1>::reference &a0) {
  return typename std::vector<bool, T1>::reference(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reference f118(std::vector<bool, T1> &a0,
                                               std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f119(typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
const typename std::vector<bool, T1>::reference &
f120(const typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f121(typename std::vector<bool, T1>::reference &a0,
     const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator=(a1);
}
