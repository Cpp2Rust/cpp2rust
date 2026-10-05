// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>
#include <streambuf>

template <typename T1>
std::istreambuf_iterator<T1> f7(std::istreambuf_iterator<T1> &a0) {
  return std::istreambuf_iterator<T1>(a0);
}

std::istreambuf_iterator<char> f8(std::basic_streambuf<char> *p) {
  return std::istreambuf_iterator<char>(p);
}
