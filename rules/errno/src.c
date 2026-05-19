// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <errno.h>

int *f1(void) { return cpp2rust_errno(); }
