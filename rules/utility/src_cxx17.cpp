// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <type_traits>
#include <utility>

template <typename T1> std::add_const_t<T1> &f7(T1 &a0) {
  return std::as_const(a0);
}
