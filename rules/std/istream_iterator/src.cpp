// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>

template <typename T1>
std::istream_iterator<T1> f6(const std::istream_iterator<T1> &a0) {
  return std::istream_iterator<T1>(a0);
}
