// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <sys/stat.h>
#endif

typedef struct stat CPP2RUST_TYPE_RULE(1);

int CPP2RUST_EXPR_RULE(1)(const char *pathname, struct stat *statbuf) {
  return stat(pathname, statbuf);
}

int CPP2RUST_EXPR_RULE(2)(int fd, struct stat *statbuf) {
  return fstat(fd, statbuf);
}

int CPP2RUST_EXPR_RULE(3)(const char *pathname, mode_t mode) {
  return mkdir(pathname, mode);
}
