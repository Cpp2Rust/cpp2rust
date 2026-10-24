// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <atomic>

template <typename T1> using t1 = std::atomic<T1>;

template <typename T1> std::atomic<T1> f1() { return {}; }

template <typename T1> T1 f2(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}
