// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

bool f1(bool condition) {
  return cpp2rust_assert_fail(condition);
}
