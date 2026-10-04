// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <unistd.h>
#endif

int CPP2RUST_EXPR_RULE(1)(int fd) { return close(fd); }

off_t CPP2RUST_EXPR_RULE(2)(int fd, off_t offset, int whence) {
  return lseek(fd, offset, whence);
}

ssize_t CPP2RUST_EXPR_RULE(3)(int fd, void *buf, size_t count) {
  return read(fd, buf, count);
}

int CPP2RUST_EXPR_RULE(4)(const char *pathname) { return unlink(pathname); }

int CPP2RUST_EXPR_RULE(5)(int pipefd[2]) { return pipe(pipefd); }

int CPP2RUST_EXPR_RULE(6)(int fd, off_t length) {
  return ftruncate(fd, length);
}

int CPP2RUST_EXPR_RULE(7)(int fd) { return isatty(fd); }

uid_t CPP2RUST_EXPR_RULE(8)(void) { return geteuid(); }

int CPP2RUST_EXPR_RULE(9)(char *name, size_t len) {
  return gethostname(name, len);
}

ssize_t CPP2RUST_EXPR_RULE(10)(int fd, const void *buf, size_t count) {
  return write(fd, buf, count);
}

int CPP2RUST_EXPR_RULE(11)(const char *pathname) { return rmdir(pathname); }
