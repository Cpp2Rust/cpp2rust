// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <map>
#include <utility>

template <typename T1, typename T2> void t1(std::map<T1, T2>);
template <typename T1, typename T2>
void t2(typename std::map<T1, T2>::const_iterator);
template <typename T1, typename T2>
void t3(typename std::map<T1, T2>::iterator);

template <typename T1, typename T2> T2 &f1(std::map<T1, T2> &o, const T1 &key) {
  return o.operator[](key);
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
std::map<T1, T2> f6(const std::map<T1, T2> &&o) {
  return std::map<T1, T2>(std::move(o));
}

template <typename T1, typename T2> T2 &f7(std::map<T1, T2> &o, const T1 &key) {
  return o.at(key);
}

template <typename T1, typename T2> T2 &f8(std::map<T1, T2> &o, T1 &&key) {
  return o.operator[](std::move(key));
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator f9(const std::map<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f10(std::map<T1, T2> &o, const T1 &key) {
  return o.find(key);
}

template <typename T1, typename T2>
bool f11(typename std::map<T1, T2>::iterator a,
         typename std::map<T1, T2>::iterator b) {
  return operator!=(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f12(std::map<T1, T2> &o) {
  return o.begin();
}

template <typename T1, typename T2>
bool f13(typename std::map<T1, T2>::const_iterator a,
         typename std::map<T1, T2>::const_iterator b) {
  return operator==(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f14(std::map<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2>
const T2 &f15(const std::map<T1, T2> &o, const T1 &key) {
  return o.at(key);
}

template <typename T1, typename T2>
bool f16(typename std::map<T1, T2>::iterator a,
         typename std::map<T1, T2>::iterator b) {
  return operator==(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator f17(const std::map<T1, T2> &o,
                                              const T1 &key) {
  return o.find(key);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator
f19(const typename std::map<T1, T2>::iterator &it) {
  return typename std::map<T1, T2>::const_iterator(it);
}

template <typename T1, typename T2>
const T1 &f20(typename std::map<T1, T2>::const_iterator it) {
  return it->first;
}

template <typename T1, typename T2>
const T2 &f21(typename std::map<T1, T2>::const_iterator it) {
  return it->second;
}

template <typename T1, typename T2>
const T1 &f22(typename std::map<T1, T2>::iterator it) {
  return it->first;
}

template <typename T1, typename T2>
T2 &f23(typename std::map<T1, T2>::iterator it) {
  return it->second;
}
