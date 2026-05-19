// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/stat.h>

int f1(const char *pathname, struct stat *statbuf) {
  return stat(pathname, statbuf);
}

int f2(int fd, struct stat *statbuf) { return fstat(fd, statbuf); }

int f3(const char *pathname, mode_t mode) { return mkdir(pathname, mode); }
