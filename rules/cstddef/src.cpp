// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <sys/types.h>

using t1 = std::byte;

typedef size_t t2;
typedef size_t *t3;
typedef const size_t *t4;
typedef decltype(sizeof(0)) t5;
typedef ssize_t t6;
typedef ssize_t *t7;
typedef const ssize_t *t8;

std::byte f1(const std::byte &a0, unsigned a1) { return operator<<(a0, a1); }

std::byte f2(const std::byte &a0, unsigned a1) { return operator>>(a0, a1); }

std::byte f3(std::byte &a0, unsigned a1) { return operator<<=(a0, a1); }

std::byte f4(std::byte &a0, unsigned a1) { return operator>>=(a0, a1); }
