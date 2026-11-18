// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <utility>

template <typename T1> const T1 &f6(T1 &a0) {
  return std::move_if_noexcept(a0);
}
