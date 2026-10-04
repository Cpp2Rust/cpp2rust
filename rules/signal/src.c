// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <signal.h>
#endif

int CPP2RUST_EXPR_RULE(1)(int signum, const struct sigaction *act,
                          struct sigaction *oldact) {
  return sigaction(signum, act, oldact);
}
