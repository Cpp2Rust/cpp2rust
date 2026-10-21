// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/types.h>
#include <unistd.h>

typedef uid_t t1;
typedef off_t t2;

#ifdef __GLIBC__
typedef __typeof__(getuid()) t3;
typedef __typeof__(lseek(0, 0, 0)) t4;
#endif
