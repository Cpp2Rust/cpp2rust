// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <utility>

template <typename T1, typename T2> auto &&f7(T2 &&a0) {
  return std::forward_like<T1>(std::forward<T2>(a0));
}
