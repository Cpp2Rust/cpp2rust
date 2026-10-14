// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>
#include <map>
#include <type_traits>
#include <utility>

#include "../common/traits.h"

template <typename T1, typename T2> using t1 = std::map<T1, T2>;

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::const_iterator>>
using t2 = It;

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::iterator>>
using t3 = It;

template <typename T1, typename T2>
using t4 = typename std::map<T1, T2>::size_type;

template <typename T1, typename T2> T2 &f1(std::map<T1, T2> &o, const T1 &key) {
  return o[key];
}

template <typename T1, typename T2> std::size_t f2(const std::map<T1, T2> &o) {
  return o.size();
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f3(std::map<T1, T2> &o,
                                       typename std::map<T1, T2>::iterator it) {
  return o.erase(it);
}

template <typename T1, typename T2> std::map<T1, T2> f5() {
  return std::map<T1, T2>();
}

template <typename T1, typename T2>
std::map<T1, T2> f6(const std::map<T1, T2> &o) {
  return std::map<T1, T2>(o);
}

template <typename T1, typename T2> T2 &f7(std::map<T1, T2> &o, const T1 &key) {
  return o.at(key);
}

template <typename T1, typename T2> T2 &f8(std::map<T1, T2> &o, T1 &&key) {
  return o[std::move(key)];
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator f9(const std::map<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f10(std::map<T1, T2> &o, const T1 &key) {
  return o.find(key);
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::iterator>>
bool f11(It a, It b) {
  return a != b;
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f12(std::map<T1, T2> &o) {
  return o.begin();
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::const_iterator>>
bool f13(It a, It b) {
  return a == b;
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f14(std::map<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2>
const T2 &f15(const std::map<T1, T2> &o, const T1 &key) {
  return o.at(key);
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::iterator>>
bool f16(It a, It b) {
  return a == b;
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator f17(const std::map<T1, T2> &o,
                                              const T1 &key) {
  return o.find(key);
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::iterator>>
typename std::map<T1, T2>::const_iterator f19(const It &it) {
  return typename std::map<T1, T2>::const_iterator(it);
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::const_iterator>>
const T1 &f20(It it) {
  return it->first;
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::const_iterator>>
const T2 &f21(It it) {
  return it->second;
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::iterator>>
const T1 &f22(It it) {
  return it->first;
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::iterator>>
T2 &f23(It it) {
  return it->second;
}

template <typename T1, typename T2> std::map<T1, T2> f24(std::map<T1, T2> &&o) {
  return std::map<T1, T2>(std::move(o));
}

template <typename T1, typename T2>
std::map<T1, T2> &f25(std::map<T1, T2> &dst, std::map<T1, T2> &&src) {
  return dst = std::move(src);
}

template <typename T1, typename T2>
std::map<T1, T2> &f26(std::map<T1, T2> &dst, const std::map<T1, T2> &src) {
  return dst = src;
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::iterator>>
std::pair<const T1, T2> &f27(It it) {
  return *it;
}

template <typename It, typename T1 = KeyOf<It>, typename T2 = MappedOf<It>,
          typename = SameAs<It, typename std::map<T1, T2>::const_iterator>>
const std::pair<const T1, T2> &f28(It it) {
  return *it;
}
