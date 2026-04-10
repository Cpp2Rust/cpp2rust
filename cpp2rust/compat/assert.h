// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include_next <assert.h>

#undef assert

#ifndef __cplusplus
#include <stdbool.h>
#endif

void __cpp2rust_assert_fail(bool condition) __attribute__((noreturn));

#define assert(expr) __cpp2rust_assert_fail(expr)
