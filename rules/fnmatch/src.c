// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <fnmatch.h>

int f1(const char *pattern, const char *string, int flags) {
  return fnmatch(pattern, string, flags);
}
