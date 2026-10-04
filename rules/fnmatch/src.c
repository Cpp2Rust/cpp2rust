// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <fnmatch.h>
#endif

int CPP2RUST_EXPR_RULE(1)(const char *pattern, const char *string, int flags) {
  return fnmatch(pattern, string, flags);
}
