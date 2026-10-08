// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>

#if __cplusplus >= 201703L
using t1 = std::byte;
#endif

typedef size_t t2;
typedef size_t *t3;
typedef const size_t *t4;
typedef decltype(sizeof(0)) t5;
typedef ssize_t t6;
typedef ssize_t *t7;
typedef const ssize_t *t8;

#if __cplusplus >= 201703L
std::byte f1(const std::byte &a0, unsigned a1) { return operator<<(a0, a1); }

std::byte f2(const std::byte &a0, unsigned a1) { return operator>>(a0, a1); }

std::byte f3(std::byte &a0, unsigned a1) { return operator<<=(a0, a1); }

std::byte f4(std::byte &a0, unsigned a1) { return operator>>=(a0, a1); }
#endif
