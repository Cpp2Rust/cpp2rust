// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include_next <errno.h>

#undef errno

int *cpp2rust_errno(void);

#define errno (*cpp2rust_errno())
