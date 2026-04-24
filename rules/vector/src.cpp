// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <initializer_list>
#include <vector>

template <typename T1> using t1 = std::vector<T1>;
template <typename T1> using t2 = typename std::vector<T1>::iterator;
template <typename T1> using t3 = std::vector<std::vector<T1>>;
template <typename T1> using t4 = typename std::vector<T1>::const_iterator;

template <typename T1>
typename std::vector<T1>::iterator
f1(std::vector<T1> &o, typename std::vector<T1>::const_iterator it) {
  return o.erase(it);
}

template <typename T1> std::size_t f2(const std::vector<T1> &o) {
  return o.size();
}
template <typename T1> bool f3(const std::vector<T1> &o) { return o.empty(); }

template <typename T1> std::vector<T1> f4() { return std::vector<T1>(); }

template <typename T1> void f5(std::vector<T1> &o) { return o.pop_back(); }

template <typename T1> T1 *f6(std::vector<T1> &o) { return o.data(); }

template <typename T1> T1 &f7(std::vector<T1> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1> std::vector<T1> f8(std::size_t n) {
  return std::vector<T1>(n);
}

template <typename T1> T1 &f9(std::vector<T1> &o) { return o.front(); }

template <typename T1> T1 &f10(std::vector<T1> &o) { return o.back(); }

template <typename T1> std::size_t f11(const std::vector<T1> &o) {
  return o.capacity();
}

template <typename T1> void f12(std::vector<T1> &o, std::size_t n) {
  return o.reserve(n);
}

template <typename T1>
typename std::vector<T1>::iterator f13(std::vector<T1> &o) {
  return o.begin();
}

template <typename T1> void f14(std::vector<T1> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1> void f15(std::vector<T1> &o, std::size_t n) {
  return o.resize(n);
}

template <typename T1> void f16(std::vector<T1> &o) { return o.clear(); }

template <typename T1>
typename std::vector<T1>::iterator f17(std::vector<T1> &o) {
  return o.end();
}

template <typename T1>
typename std::vector<T1>::iterator
f18(std::vector<T1> &o, typename std::vector<T1>::const_iterator it,
    T1 &&value) {
  return o.insert(it, std::move(value));
}

template <typename T1> std::vector<T1> f19(std::size_t n, const T1 &value) {
  return std::vector<T1>(n, value);
}

template <typename T1>
typename std::vector<T1>::iterator
f20(std::vector<T1> &o, typename std::vector<T1>::const_iterator it,
    const T1 &value) {
  return o.insert(it, value);
}

template <typename T1> void f21(std::vector<T1> &o, const T1 &value) {
  return o.push_back(value);
}

template <typename T1>
typename std::vector<T1>::reference f22(typename std::vector<T1>::iterator it) {
  return it.operator*();
}

template <typename T1>
typename std::vector<T1>::iterator
f23(const typename std::vector<T1>::iterator &it) {
  return typename std::vector<T1>::iterator(it);
}

template <typename T1>
typename std::vector<T1>::const_iterator
f24(const typename std::vector<T1>::iterator &it) {
  return typename std::vector<T1>::const_iterator(it);
}

template <typename T1>
typename std::vector<T1>::iterator f25(typename std::vector<T1>::iterator it,
                                       std::size_t n) {
  return it.operator+(n);
}

template <typename T1>
bool f26(const typename std::vector<T1>::iterator &it1,
         const typename std::vector<T1>::iterator &it2) {
  return operator!=(it1, it2);
}

template <typename T1>
bool f27(const typename std::vector<T1>::iterator &it1,
         const typename std::vector<T1>::iterator &it2) {
  return operator==(it1, it2);
}

template <typename T1>
typename std::vector<T1>::iterator f28(typename std::vector<T1>::iterator a0,
                                       int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::vector<std::vector<T1>> f29(const std::vector<std::vector<T1>> &&o) {
  return std::vector<std::vector<T1>>(std::move(o));
}

template <typename T1> std::vector<std::vector<T1>> f30(std::size_t n) {
  return std::vector<std::vector<T1>>(n);
}

template <typename T1>
void f31(std::vector<std::vector<T1>> &o, std::vector<T1> &&value) {
  return o.push_back(std::move(value));
}

template <typename T1>
void f32(std::vector<std::vector<T1>> &o, std::size_t n) {
  return o.resize(n);
}

template <typename T1>
typename std::vector<T1>::iterator::difference_type
f33(const typename std::vector<T1>::iterator &it1,
    const typename std::vector<T1>::iterator &it2) {
  return operator-(it1, it2);
}

template <typename T1>
typename std::vector<T1>::iterator &
f34(typename std::vector<T1>::iterator &it) {
  return it.operator++();
}

template <typename T1> std::vector<T1> f35(const T1 *first, const T1 *last) {
  return std::vector<T1>(first, last);
}

template <typename T1>
std::vector<T1> f36(const std::initializer_list<T1> &a0) {
  return std::vector<T1>(a0);
}

template <typename T1> std::vector<T1> f37(T1 *first, T1 *last) {
  return std::vector<T1>(first, last);
}

std::vector<bool> f38(std::size_t n, const bool &value) {
  return std::vector<bool>(n, value);
}

std::vector<int> f39(unsigned int *first, unsigned int *last) {
  return std::vector<int>(first, last);
}

template <class T1, std::size_t T2> const T1 *f40(T1 const (&a0)[T2]) {
  return std::end(a0);
}

template <typename T1> const T1 *f41(const std::vector<T1> &o) {
  return o.data();
}

template <typename T1>
typename std::vector<T1>::const_iterator
f42(typename std::vector<T1>::const_iterator first,
    typename std::vector<T1>::const_iterator last) {
  return std::max_element(first, last);
}

template <typename T1>
typename std::vector<T1>::const_iterator f43(const std::vector<T1> &o) {
  return o.begin();
}

template <typename T1>
typename std::vector<T1>::const_iterator f44(const std::vector<T1> &o) {
  return o.end();
}

bool f47(std::vector<bool> &o) { return o[0]; }

template <typename T1> void f48(std::vector<T1> &o, std::vector<T1> &a0) {
  return o.swap(a0);
}

template <typename T1>
const T1 &f50(const std::vector<T1> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1> const T1 &f51(const std::vector<T1> &o) {
  return o.back();
}

template <typename T1>
void f52(std::vector<std::vector<T1>> &o, const std::vector<T1> &value) {
  return o.push_back(value);
}

template <typename T1>
typename std::vector<T1>::iterator
f53(std::vector<T1> &o, typename std::vector<T1>::const_iterator pos,
    const T1 *first, const T1 *last) {
  return o.insert(pos, first, last);
}

template <typename T1>
void f54(std::vector<T1> &o, std::size_t n,
         const typename std::vector<T1>::value_type &value) {
  return o.resize(n, value);
}

template <typename T1>
std::vector<T1> &f55(std::vector<T1> &dst, std::vector<T1> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1> std::vector<T1> &f56(std::vector<std::vector<T1>> &o) {
  return o.back();
}

template <typename T1>
typename std::vector<T1>::const_iterator f57(const std::vector<T1> &o) {
  return o.cend();
}

template <typename T1>
std::vector<T1> &f58(std::vector<T1> &dst, const std::vector<T1> &src) {
  return dst.operator=(src);
}

template <typename T1> void f59(std::vector<T1> &o) {
  return o.shrink_to_fit();
}
