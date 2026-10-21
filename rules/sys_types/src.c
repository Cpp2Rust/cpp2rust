// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/types.h>

typedef uid_t t1;
typedef off_t t2;

#ifdef __GLIBC__
typedef __uid_t t3;
typedef __off_t t4;
#endif
