// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <map>
#include <utility>
#endif

template <typename T1, typename T2>
using CPP2RUST_TYPE_RULE(1) = std::map<T1, T2>;

template <typename T1, typename T2>
using CPP2RUST_TYPE_RULE(2) = typename std::map<T1, T2>::const_iterator;

template <typename T1, typename T2>
using CPP2RUST_TYPE_RULE(3) = typename std::map<T1, T2>::iterator;

template <typename T1, typename T2>
T2 &CPP2RUST_EXPR_RULE(1)(std::map<T1, T2> &o, const T1 &key) {
  return o.operator[](key);
}

template <typename T1, typename T2>
std::size_t CPP2RUST_EXPR_RULE(2)(const std::map<T1, T2> &o) {
  return o.size();
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator
CPP2RUST_EXPR_RULE(3)(std::map<T1, T2> &o,
                      typename std::map<T1, T2>::iterator it) {
  return o.erase(it);
}

template <typename T1, typename T2> std::map<T1, T2> CPP2RUST_EXPR_RULE(5)() {
  return std::map<T1, T2>();
}

template <typename T1, typename T2>
std::map<T1, T2> CPP2RUST_EXPR_RULE(6)(const std::map<T1, T2> &&o) {
  return std::map<T1, T2>(std::move(o));
}

template <typename T1, typename T2>
T2 &CPP2RUST_EXPR_RULE(7)(std::map<T1, T2> &o, const T1 &key) {
  return o.at(key);
}

template <typename T1, typename T2>
T2 &CPP2RUST_EXPR_RULE(8)(std::map<T1, T2> &o, T1 &&key) {
  return o.operator[](std::move(key));
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator
CPP2RUST_EXPR_RULE(9)(const std::map<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator CPP2RUST_EXPR_RULE(10)(std::map<T1, T2> &o,
                                                           const T1 &key) {
  return o.find(key);
}

template <typename T1, typename T2>
bool CPP2RUST_EXPR_RULE(11)(typename std::map<T1, T2>::iterator a,
                            typename std::map<T1, T2>::iterator b) {
  return operator!=(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator
CPP2RUST_EXPR_RULE(12)(std::map<T1, T2> &o) {
  return o.begin();
}

template <typename T1, typename T2>
bool CPP2RUST_EXPR_RULE(13)(typename std::map<T1, T2>::const_iterator a,
                            typename std::map<T1, T2>::const_iterator b) {
  return operator==(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator
CPP2RUST_EXPR_RULE(14)(std::map<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2>
const T2 &CPP2RUST_EXPR_RULE(15)(const std::map<T1, T2> &o, const T1 &key) {
  return o.at(key);
}

template <typename T1, typename T2>
bool CPP2RUST_EXPR_RULE(16)(typename std::map<T1, T2>::iterator a,
                            typename std::map<T1, T2>::iterator b) {
  return operator==(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator
CPP2RUST_EXPR_RULE(17)(const std::map<T1, T2> &o, const T1 &key) {
  return o.find(key);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator
CPP2RUST_EXPR_RULE(19)(const typename std::map<T1, T2>::iterator &it) {
  return typename std::map<T1, T2>::const_iterator(it);
}

template <typename T1, typename T2>
const T1 &CPP2RUST_EXPR_RULE(20)(typename std::map<T1, T2>::const_iterator it) {
  return it->first;
}

template <typename T1, typename T2>
const T2 &CPP2RUST_EXPR_RULE(21)(typename std::map<T1, T2>::const_iterator it) {
  return it->second;
}

template <typename T1, typename T2>
const T1 &CPP2RUST_EXPR_RULE(22)(typename std::map<T1, T2>::iterator it) {
  return it->first;
}

template <typename T1, typename T2>
T2 &CPP2RUST_EXPR_RULE(23)(typename std::map<T1, T2>::iterator it) {
  return it->second;
}

template <typename T1, typename T2>
std::map<T1, T2> CPP2RUST_EXPR_RULE(24)(std::map<T1, T2> &&o) {
  return std::map<T1, T2>(std::move(o));
}

template <typename T1, typename T2>
std::map<T1, T2> &CPP2RUST_EXPR_RULE(25)(std::map<T1, T2> &dst,
                                         std::map<T1, T2> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1, typename T2>
std::map<T1, T2> &CPP2RUST_EXPR_RULE(26)(std::map<T1, T2> &dst,
                                         const std::map<T1, T2> &src) {
  return dst.operator=(src);
}
