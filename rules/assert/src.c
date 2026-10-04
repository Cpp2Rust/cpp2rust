// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <assert.h>
#endif

bool CPP2RUST_EXPR_RULE(1)(bool condition) {
  return cpp2rust_assert_fail(condition);
}
