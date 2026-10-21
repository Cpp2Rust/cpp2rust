// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/types.h>

typedef uid_t t1;
typedef off_t t2;

// glibc declares some APIs with private typedefs, e.g. `__off_t lseek(...)`,
// where Darwin uses `off_t`. Give them the public typedef's translation so
// both platforms produce the same output.
//
// TODO:
//
//   Rules are written in terms of standard names `off_t lseek(...)`, however
//   libraries choose to implement them using library specific names `__off_t
//   lseek(...)`.
//
//   Fix this in converter by using the names in the rules, not the
//   library-specific names.

#ifdef __GLIBC__
typedef __uid_t t3;
typedef __off_t t4;
#endif
