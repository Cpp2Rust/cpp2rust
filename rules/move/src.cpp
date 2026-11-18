// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <utility>

template <typename T1> T1 &&f2(typename std::remove_reference<T1>::type &&a0) {
  return std::forward<T1>(std::move(a0));
}

template <typename T1> T1 &&f3(typename std::remove_reference<T1>::type &a0) {
  return std::forward<T1>(a0);
}

template <typename T1> typename std::remove_reference<T1>::type &&f5(T1 &&a0) {
  return std::move(std::forward<T1>(a0));
}
