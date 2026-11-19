// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <deque>
#include <utility>

template <typename T1, typename... T2> void f15(std::deque<T1> &o, T2 &&...a1) {
  return o.emplace_back(std::forward<T2>(a1)...);
}
