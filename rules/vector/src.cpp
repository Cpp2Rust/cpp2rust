// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <algorithm>
#include <initializer_list>
#include <vector>
#endif

template <typename T, typename A> using Init = A;

template <typename T1> using CPP2RUST_TYPE_RULE(1) = std::vector<T1>;
template <typename T1>
using CPP2RUST_TYPE_RULE(2) = typename std::vector<T1>::iterator;
template <typename T1>
using CPP2RUST_TYPE_RULE(3) = std::vector<std::vector<T1>>;
template <typename T1>
using CPP2RUST_TYPE_RULE(4) = typename std::vector<T1>::const_iterator;

template <typename T1, typename T2 = std::allocator<T1>>
using CPP2RUST_TYPE_RULE(5) = std::vector<T1, T2>;

#if defined(__linux__)
template <typename T1, typename T2 = std::allocator<T1>>
using CPP2RUST_TYPE_RULE(6) = typename std::vector<T1, T2>::iterator;
template <typename T1, typename T2 = std::allocator<T1>>
using CPP2RUST_TYPE_RULE(7) = typename std::vector<T1, T2>::const_iterator;
#endif

template <typename T1>
typename std::vector<T1>::iterator
CPP2RUST_EXPR_RULE(1)(std::vector<T1> &o,
                      typename std::vector<T1>::const_iterator it) {
  return o.erase(it);
}

template <typename T1>
std::size_t CPP2RUST_EXPR_RULE(2)(const std::vector<T1> &o) {
  return o.size();
}
template <typename T1> bool CPP2RUST_EXPR_RULE(3)(const std::vector<T1> &o) {
  return o.empty();
}

template <typename T1> std::vector<T1> CPP2RUST_EXPR_RULE(4)() {
  return std::vector<T1>();
}

template <typename T1> void CPP2RUST_EXPR_RULE(5)(std::vector<T1> &o) {
  return o.pop_back();
}

template <typename T1> T1 *CPP2RUST_EXPR_RULE(6)(std::vector<T1> &o) {
  return o.data();
}

template <typename T1>
T1 &CPP2RUST_EXPR_RULE(7)(std::vector<T1> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1> std::vector<T1> CPP2RUST_EXPR_RULE(8)(std::size_t n) {
  return std::vector<T1>(n);
}

template <typename T1> T1 &CPP2RUST_EXPR_RULE(9)(std::vector<T1> &o) {
  return o.front();
}

template <typename T1> T1 &CPP2RUST_EXPR_RULE(10)(std::vector<T1> &o) {
  return o.back();
}

template <typename T1>
std::size_t CPP2RUST_EXPR_RULE(11)(const std::vector<T1> &o) {
  return o.capacity();
}

template <typename T1>
void CPP2RUST_EXPR_RULE(12)(std::vector<T1> &o, std::size_t n) {
  return o.reserve(n);
}

template <typename T1>
typename std::vector<T1>::iterator CPP2RUST_EXPR_RULE(13)(std::vector<T1> &o) {
  return o.begin();
}

template <typename T1>
void CPP2RUST_EXPR_RULE(14)(std::vector<T1> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1>
void CPP2RUST_EXPR_RULE(15)(std::vector<T1> &o, std::size_t n) {
  return o.resize(n);
}

template <typename T1> void CPP2RUST_EXPR_RULE(16)(std::vector<T1> &o) {
  return o.clear();
}

template <typename T1>
typename std::vector<T1>::iterator CPP2RUST_EXPR_RULE(17)(std::vector<T1> &o) {
  return o.end();
}

template <typename T1>
typename std::vector<T1>::iterator
CPP2RUST_EXPR_RULE(18)(std::vector<T1> &o,
                       typename std::vector<T1>::const_iterator it,
                       T1 &&value) {
  return o.insert(it, std::move(value));
}

template <typename T1>
std::vector<T1> CPP2RUST_EXPR_RULE(19)(std::size_t n, const T1 &value) {
  return std::vector<T1>(n, value);
}

template <typename T1>
typename std::vector<T1>::iterator
CPP2RUST_EXPR_RULE(20)(std::vector<T1> &o,
                       typename std::vector<T1>::const_iterator it,
                       const T1 &value) {
  return o.insert(it, value);
}

template <typename T1>
void CPP2RUST_EXPR_RULE(21)(std::vector<T1> &o, const T1 &value) {
  return o.push_back(value);
}

template <typename T1>
typename std::vector<T1>::reference
CPP2RUST_EXPR_RULE(22)(typename std::vector<T1>::iterator it) {
  return it.operator*();
}

template <typename T1>
typename std::vector<T1>::iterator
CPP2RUST_EXPR_RULE(23)(const typename std::vector<T1>::iterator &it) {
  return typename std::vector<T1>::iterator(it);
}

template <typename T1>
typename std::vector<T1>::const_iterator
CPP2RUST_EXPR_RULE(24)(const typename std::vector<T1>::iterator &it) {
  return typename std::vector<T1>::const_iterator(it);
}

template <typename T1>
typename std::vector<T1>::iterator
CPP2RUST_EXPR_RULE(25)(typename std::vector<T1>::iterator it, std::size_t n) {
  return it.operator+(n);
}

template <typename T1>
bool CPP2RUST_EXPR_RULE(26)(const typename std::vector<T1>::iterator &it1,
                            const typename std::vector<T1>::iterator &it2) {
  return operator!=(it1, it2);
}

template <typename T1>
bool CPP2RUST_EXPR_RULE(27)(const typename std::vector<T1>::iterator &it1,
                            const typename std::vector<T1>::iterator &it2) {
  return operator==(it1, it2);
}

template <typename T1>
typename std::vector<T1>::iterator
CPP2RUST_EXPR_RULE(28)(typename std::vector<T1>::iterator a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::vector<std::vector<T1>>
CPP2RUST_EXPR_RULE(29)(const std::vector<std::vector<T1>> &&o) {
  return std::vector<std::vector<T1>>(std::move(o));
}

template <typename T1>
std::vector<std::vector<T1>> CPP2RUST_EXPR_RULE(30)(std::size_t n) {
  return std::vector<std::vector<T1>>(n);
}

template <typename T1>
void CPP2RUST_EXPR_RULE(31)(std::vector<std::vector<T1>> &o,
                            std::vector<T1> &&value) {
  return o.push_back(std::move(value));
}

template <typename T1>
void CPP2RUST_EXPR_RULE(32)(std::vector<std::vector<T1>> &o, std::size_t n) {
  return o.resize(n);
}

template <typename T1>
typename std::vector<T1>::iterator::difference_type
CPP2RUST_EXPR_RULE(33)(const typename std::vector<T1>::iterator &it1,
                       const typename std::vector<T1>::iterator &it2) {
  return operator-(it1, it2);
}

template <typename T1>
typename std::vector<T1>::iterator &
CPP2RUST_EXPR_RULE(34)(typename std::vector<T1>::iterator &it) {
  return it.operator++();
}

template <typename T1>
std::vector<T1> CPP2RUST_EXPR_RULE(35)(const T1 *first, const T1 *last) {
  return std::vector<T1>(first, last);
}

template <typename T1>
std::vector<T1> CPP2RUST_EXPR_RULE(36)(const std::initializer_list<T1> &a0) {
  return std::vector<T1>(a0);
}

template <typename T1, typename T2>
std::vector<T1> CPP2RUST_EXPR_RULE(37)(T2 *first, T2 *last) {
  return std::vector<T1>(first, last);
}

std::vector<bool> CPP2RUST_EXPR_RULE(38)(std::size_t n, const bool &value) {
  return std::vector<bool>(n, value);
}

template <class T1, std::size_t T2>
const T1 *CPP2RUST_EXPR_RULE(40)(T1 const (&a0)[T2]) {
  return std::end(a0);
}

template <typename T1>
const T1 *CPP2RUST_EXPR_RULE(41)(const std::vector<T1> &o) {
  return o.data();
}

template <typename T1>
typename std::vector<T1>::const_iterator
CPP2RUST_EXPR_RULE(42)(typename std::vector<T1>::const_iterator first,
                       typename std::vector<T1>::const_iterator last) {
  return std::max_element(first, last);
}

template <typename T1>
typename std::vector<T1>::const_iterator
CPP2RUST_EXPR_RULE(43)(const std::vector<T1> &o) {
  return o.begin();
}

template <typename T1>
typename std::vector<T1>::const_iterator
CPP2RUST_EXPR_RULE(44)(const std::vector<T1> &o) {
  return o.end();
}

bool CPP2RUST_EXPR_RULE(47)(std::vector<bool> &o) { return o[0]; }

template <typename T1>
void CPP2RUST_EXPR_RULE(48)(std::vector<T1> &o, std::vector<T1> &a0) {
  return o.swap(a0);
}

template <typename T1>
const T1 &CPP2RUST_EXPR_RULE(50)(const std::vector<T1> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1>
const T1 &CPP2RUST_EXPR_RULE(51)(const std::vector<T1> &o) {
  return o.back();
}

template <typename T1>
void CPP2RUST_EXPR_RULE(52)(std::vector<std::vector<T1>> &o,
                            const std::vector<T1> &value) {
  return o.push_back(value);
}

template <typename T1>
typename std::vector<T1>::iterator
CPP2RUST_EXPR_RULE(53)(std::vector<T1> &o,
                       typename std::vector<T1>::const_iterator pos,
                       const T1 *first, const T1 *last) {
  return o.insert(pos, first, last);
}

template <typename T1>
void CPP2RUST_EXPR_RULE(54)(std::vector<T1> &o, std::size_t n,
                            const typename std::vector<T1>::value_type &value) {
  return o.resize(n, value);
}

template <typename T1>
std::vector<T1> &CPP2RUST_EXPR_RULE(55)(std::vector<T1> &dst,
                                        std::vector<T1> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1>
std::vector<T1> &CPP2RUST_EXPR_RULE(56)(std::vector<std::vector<T1>> &o) {
  return o.back();
}

template <typename T1>
typename std::vector<T1>::const_iterator
CPP2RUST_EXPR_RULE(57)(const std::vector<T1> &o) {
  return o.cend();
}

template <typename T1>
std::vector<T1> &CPP2RUST_EXPR_RULE(58)(std::vector<T1> &dst,
                                        const std::vector<T1> &src) {
  return dst.operator=(src);
}

template <typename T1> void CPP2RUST_EXPR_RULE(59)(std::vector<T1> &o) {
  return o.shrink_to_fit();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(60)(std::vector<T1, T2> &o,
                       typename std::vector<T1, T2>::const_iterator it) {
  return o.erase(it);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::size_t CPP2RUST_EXPR_RULE(61)(const std::vector<T1, T2> &o) {
  return o.size();
}

template <typename T1, typename T2 = std::allocator<T1>>
bool CPP2RUST_EXPR_RULE(62)(const std::vector<T1, T2> &o) {
  return o.empty();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> CPP2RUST_EXPR_RULE(63)() {
  return std::vector<T1, T2>();
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(64)(std::vector<T1, T2> &o) {
  return o.pop_back();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 *CPP2RUST_EXPR_RULE(65)(std::vector<T1, T2> &o) {
  return o.data();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &CPP2RUST_EXPR_RULE(66)(std::vector<T1, T2> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> CPP2RUST_EXPR_RULE(67)(std::size_t n) {
  return std::vector<T1, T2>(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &CPP2RUST_EXPR_RULE(68)(std::vector<T1, T2> &o) {
  return o.front();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &CPP2RUST_EXPR_RULE(69)(std::vector<T1, T2> &o) {
  return o.back();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::size_t CPP2RUST_EXPR_RULE(70)(const std::vector<T1, T2> &o) {
  return o.capacity();
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(71)(std::vector<T1, T2> &o, std::size_t n) {
  return o.reserve(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(72)(std::vector<T1, T2> &o) {
  return o.begin();
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(73)(std::vector<T1, T2> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(74)(std::vector<T1, T2> &o, std::size_t n) {
  return o.resize(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(75)(std::vector<T1, T2> &o) {
  return o.clear();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(76)(std::vector<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(77)(std::vector<T1, T2> &o,
                       typename std::vector<T1, T2>::const_iterator it,
                       T1 &&value) {
  return o.insert(it, std::move(value));
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> CPP2RUST_EXPR_RULE(78)(std::size_t n, const T1 &value) {
  return std::vector<T1, T2>(n, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(79)(std::vector<T1, T2> &o,
                       typename std::vector<T1, T2>::const_iterator it,
                       const T1 &value) {
  return o.insert(it, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(80)(std::vector<T1, T2> &o, const T1 &value) {
  return o.push_back(value);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::reference
CPP2RUST_EXPR_RULE(81)(typename std::vector<T1, T2>::iterator it) {
  return it.operator*();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(82)(const typename std::vector<T1, T2>::iterator &it) {
  return typename std::vector<T1, T2>::iterator(it);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator
CPP2RUST_EXPR_RULE(83)(const typename std::vector<T1, T2>::iterator &it) {
  return typename std::vector<T1, T2>::const_iterator(it);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(84)(typename std::vector<T1, T2>::iterator it,
                       std::size_t n) {
  return it.operator+(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
bool CPP2RUST_EXPR_RULE(85)(const typename std::vector<T1, T2>::iterator &it1,
                            const typename std::vector<T1, T2>::iterator &it2) {
  return operator!=(it1, it2);
}

template <typename T1, typename T2 = std::allocator<T1>>
bool CPP2RUST_EXPR_RULE(86)(const typename std::vector<T1, T2>::iterator &it1,
                            const typename std::vector<T1, T2>::iterator &it2) {
  return operator==(it1, it2);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(87)(typename std::vector<T1, T2>::iterator a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator::difference_type
CPP2RUST_EXPR_RULE(88)(const typename std::vector<T1, T2>::iterator &it1,
                       const typename std::vector<T1, T2>::iterator &it2) {
  return operator-(it1, it2);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator &
CPP2RUST_EXPR_RULE(89)(typename std::vector<T1, T2>::iterator &it) {
  return it.operator++();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> CPP2RUST_EXPR_RULE(90)(const T1 *first, const T1 *last) {
  return std::vector<T1, T2>(first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2>
CPP2RUST_EXPR_RULE(91)(const std::initializer_list<T1> &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2 = std::allocator<T1>, typename T3>
std::vector<T1, T2> CPP2RUST_EXPR_RULE(92)(T3 *first, T3 *last) {
  return std::vector<T1, T2>(first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 *CPP2RUST_EXPR_RULE(93)(const std::vector<T1, T2> &o) {
  return o.data();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator
CPP2RUST_EXPR_RULE(94)(typename std::vector<T1, T2>::const_iterator first,
                       typename std::vector<T1, T2>::const_iterator last) {
  return std::max_element(first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator
CPP2RUST_EXPR_RULE(95)(const std::vector<T1, T2> &o) {
  return o.begin();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator
CPP2RUST_EXPR_RULE(96)(const std::vector<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(97)(std::vector<T1, T2> &o, std::vector<T1, T2> &a0) {
  return o.swap(a0);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 &CPP2RUST_EXPR_RULE(98)(const std::vector<T1, T2> &o,
                                 std::size_t idx) {
  return o.at(idx);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 &CPP2RUST_EXPR_RULE(99)(const std::vector<T1, T2> &o) {
  return o.back();
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(100)(std::vector<std::vector<T1, T2>> &o,
                             const std::vector<T1, T2> &value) {
  return o.push_back(value);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
CPP2RUST_EXPR_RULE(101)(std::vector<T1, T2> &o,
                        typename std::vector<T1, T2>::const_iterator pos,
                        const T1 *first, const T1 *last) {
  return o.insert(pos, first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(102)(
    std::vector<T1, T2> &o, std::size_t n,
    const typename std::vector<T1, T2>::value_type &value) {
  return o.resize(n, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> &CPP2RUST_EXPR_RULE(103)(std::vector<T1, T2> &dst,
                                             std::vector<T1, T2> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator
CPP2RUST_EXPR_RULE(104)(const std::vector<T1, T2> &o) {
  return o.cend();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> &CPP2RUST_EXPR_RULE(105)(std::vector<T1, T2> &dst,
                                             const std::vector<T1, T2> &src) {
  return dst.operator=(src);
}

template <typename T1, typename T2 = std::allocator<T1>>
void CPP2RUST_EXPR_RULE(106)(std::vector<T1, T2> &o) {
  return o.shrink_to_fit();
}

template <typename T1>
std::vector<T1> CPP2RUST_EXPR_RULE(107)(std::vector<T1> &&o) {
  return std::vector<T1>(std::move(o));
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> CPP2RUST_EXPR_RULE(108)(std::vector<T1, T2> &&o) {
  return std::vector<T1, T2>(std::move(o));
}

template <typename T1>
std::vector<T1> CPP2RUST_EXPR_RULE(109)(const std::vector<T1> &o) {
  return std::vector<T1>(o);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> CPP2RUST_EXPR_RULE(110)(const std::vector<T1, T2> &o) {
  return std::vector<T1, T2>(o);
}

template <typename T1>
std::vector<std::vector<T1>> &
CPP2RUST_EXPR_RULE(111)(std::vector<std::vector<T1>> &dst,
                        std::vector<std::vector<T1>> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1, typename... Args>
T1 &CPP2RUST_EXPR_RULE(112)(std::vector<T1> &o, Init<T1, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}

template <typename T1, typename... Args>
std::vector<T1> &
CPP2RUST_EXPR_RULE(113)(std::vector<std::vector<T1>> &o,
                        Init<std::vector<T1>, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}

template <typename T1, typename T2 = std::allocator<T1>, typename... Args>
T1 &CPP2RUST_EXPR_RULE(114)(std::vector<T1, T2> &o, Init<T1, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}
