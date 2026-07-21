// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <fcntl.h>

template <typename... Args>
int f1(int a0, int a1, Args... args) {
  return fcntl(a0, a1, args...);
}

template <typename... Args>
int f2(const char *a0, int a1, Args... args) {
  return open(a0, a1, args...);
}

int f3(void) { return O_CREAT; }
int f4(void) { return O_TRUNC; }
int f5(void) { return O_APPEND; }
int f6(void) { return O_EXCL; }
int f7(void) { return O_NONBLOCK; }
int f8(void) { return O_CLOEXEC; }
int f9(void) { return O_RDONLY; }
int f10(void) { return O_WRONLY; }
int f11(void) { return O_RDWR; }
