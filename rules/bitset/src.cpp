// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <bitset>

template <std::size_t T1> using t1 = std::bitset<T1>;

template <std::size_t T1> std::bitset<T1> f1() { return std::bitset<T1>(); }

template <std::size_t T1> std::bitset<T1> f2(unsigned long long a0) {
  return std::bitset<T1>(a0);
}

template <std::size_t T1>
std::bitset<T1> f3(const std::bitset<T1> &a0, std::size_t a1) {
  return a0.operator<<(a1);
}

template <std::size_t T1>
std::bitset<T1> f4(const std::bitset<T1> &a0, std::size_t a1) {
  return a0.operator>>(a1);
}

template <std::size_t T1> unsigned long f5(const std::bitset<T1> &a0) {
  return a0.to_ulong();
}
