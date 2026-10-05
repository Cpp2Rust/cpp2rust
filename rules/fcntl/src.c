// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#define _GNU_SOURCE
#include <fcntl.h>

int (*f1)(int, int, ...) = fcntl;

int (*f2)(const char *, int, ...) = open;

int f3(void) { return O_CREAT; }
int f4(void) { return O_TRUNC; }
int f5(void) { return O_APPEND; }
int f6(void) { return O_EXCL; }
int f7(void) { return O_NONBLOCK; }
int f8(void) { return O_CLOEXEC; }
int f9(void) { return O_RDONLY; }
int f10(void) { return O_WRONLY; }
int f11(void) { return O_RDWR; }
