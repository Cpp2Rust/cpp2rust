// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <deque>
#include <vector>

template <typename T1> using t1 = std::deque<T1>;

template <typename T1> using t2 = typename std::deque<T1>::size_type;

template <typename T1> using t3 = typename std::deque<T1>::value_type;

template <typename T1> using t4 = typename std::deque<T1>::reference;

template <typename T1> using t5 = typename std::deque<T1>::const_reference;

template <typename T1> T1 &f1(std::deque<T1> &o) { return o.back(); }

template <typename T1> T1 &f2(std::deque<T1> &o) { return o.front(); }

template <typename T1> bool f3(const std::deque<T1> &o) { return o.empty(); }

template <typename T1> void f4(std::deque<T1> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1> void f5(std::deque<T1> &o) { return o.pop_front(); }

template <typename T1> void f7(std::deque<T1> &o, const T1 &value) {
  return o.push_back(value);
}

template <typename T1> std::deque<T1> f8(const std::deque<T1> &o) {
  return std::deque<T1>(o);
}

template <typename T1> std::deque<T1> f9(std::deque<T1> &&o) {
  return std::deque<T1>(std::move(o));
}

template <typename T1>
std::deque<T1> &f10(std::deque<T1> &dst, const std::deque<T1> &src) {
  return dst = src;
}

template <typename T1>
std::deque<T1> &f11(std::deque<T1> &dst, std::deque<T1> &&src) {
  return dst = std::move(src);
}

template <typename T1, typename... Args>
auto f12(std::deque<T1> &o, Args &&...args)
    -> decltype(o.emplace_back(std::forward<Args>(args)...)) {
  return o.emplace_back(std::forward<Args>(args)...);
}

template <typename T1> std::deque<T1> f14() { return std::deque<T1>(); }
