// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <deque>
#include <vector>

template <typename T1> using t1 = std::deque<T1>;

template <typename T1> T1 &f1(std::deque<T1> &o) { return o.back(); }

template <typename T1> T1 &f2(std::deque<T1> &o) { return o.front(); }

template <typename T1> bool f3(const std::deque<T1> &o) { return o.empty(); }

template <typename T1> void f4(std::deque<T1> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1> void f5(std::deque<T1> &o) { return o.pop_front(); }

template <typename T1>
void f7(std::deque<std::vector<T1>> &o, const std::vector<T1> &value) {
  return o.push_back(value);
}
