// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>

#include "rule_hints.h"

using t1 = std::byte;

template <typename T1 = Integer> std::byte f1(const std::byte &a0, T1 a1) {
  return operator<<(a0, a1);
}

template <typename T1 = Integer> std::byte f2(const std::byte &a0, T1 a1) {
  return operator>>(a0, a1);
}

template <typename T1 = Integer> std::byte f3(std::byte &a0, T1 a1) {
  return operator<<=(a0, a1);
}

template <typename T1 = Integer> std::byte f4(std::byte &a0, T1 a1) {
  return operator>>=(a0, a1);
}
