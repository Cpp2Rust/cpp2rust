// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <memory>
#include <utility>
#include <vector>

template <typename T1, typename T2 = std::allocator<T1>, typename... T3>
T1 &f114(std::vector<T1, T2> &o, T3 &&...a1) {
  return o.emplace_back(std::forward<T3>(a1)...);
}
