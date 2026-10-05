// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>
#include <iterator>

using t3 = std::ostream_iterator<char>;

template <typename T1>
std::ostream_iterator<T1> f2(const std::ostream_iterator<T1> &a0) {
  return std::ostream_iterator<T1>(a0);
}

template <typename T1> std::ostream_iterator<T1> f3(std::ostream &a0) {
  return std::ostream_iterator<T1>(a0);
}
