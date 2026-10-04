// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <fcntl.h>
#endif

int (*CPP2RUST_EXPR_RULE(1))(int, int, ...) = fcntl;

int (*CPP2RUST_EXPR_RULE(2))(const char *, int, ...) = open;

int CPP2RUST_EXPR_RULE(3)(void) { return O_CREAT; }
int CPP2RUST_EXPR_RULE(4)(void) { return O_TRUNC; }
int CPP2RUST_EXPR_RULE(5)(void) { return O_APPEND; }
int CPP2RUST_EXPR_RULE(6)(void) { return O_EXCL; }
int CPP2RUST_EXPR_RULE(7)(void) { return O_NONBLOCK; }
int CPP2RUST_EXPR_RULE(8)(void) { return O_CLOEXEC; }
int CPP2RUST_EXPR_RULE(9)(void) { return O_RDONLY; }
int CPP2RUST_EXPR_RULE(10)(void) { return O_WRONLY; }
int CPP2RUST_EXPR_RULE(11)(void) { return O_RDWR; }
