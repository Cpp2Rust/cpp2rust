// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>

using t1 = std::byte;

std::byte f1(const std::byte &a0, unsigned a1) { return a0 << a1; }

std::byte f2(const std::byte &a0, unsigned a1) { return a0 >> a1; }

std::byte f3(std::byte &a0, unsigned a1) { return a0 <<= a1; }

std::byte f4(std::byte &a0, unsigned a1) { return a0 >>= a1; }

template <typename T1> std::byte f5(std::byte a0, T1 a1) { return a0 << a1; }
