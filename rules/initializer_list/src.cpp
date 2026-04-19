// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <initializer_list>

template <typename T1> void t1(std::initializer_list<T1>);

template <typename T1>
typename std::initializer_list<T1>::size_type f1(std::initializer_list<T1> &o) {
  return o.size();
}
