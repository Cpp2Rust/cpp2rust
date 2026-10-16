// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>

template <typename T1> void f18(T1 first, T1 last) {
  return std::random_shuffle(first, last);
}
