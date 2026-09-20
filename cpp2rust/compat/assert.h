// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include_next <assert.h>

#undef assert

#ifdef __cplusplus
#ifndef CPP2RUST_ASSERT_FAIL_DEFINED
#define CPP2RUST_ASSERT_FAIL_DEFINED
constexpr void cpp2rust_assert_fail(bool condition) {
  if (!condition) {
    __builtin_trap();
  }
}
#endif // CPP2RUST_ASSERT_FAIL_DEFINED
#define assert(expr) cpp2rust_assert_fail(static_cast<bool>(expr))
#else
#include <stdbool.h>
void cpp2rust_assert_fail(bool condition) __attribute__((noreturn));
#define assert(expr) cpp2rust_assert_fail(expr)
#endif // __cplusplus
