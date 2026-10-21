// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <stdatomic.h>

void f1(volatile atomic_flag *a0) { return atomic_flag_clear(a0); }
