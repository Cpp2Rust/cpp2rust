// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include_next <assert.h>

#undef assert

void __cpp2rust_assert_fail(int condition) __attribute__((noreturn));

#define assert(expr) __cpp2rust_assert_fail(expr)
