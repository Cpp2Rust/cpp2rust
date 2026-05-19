// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <signal.h>

int f1(int signum, const struct sigaction *act, struct sigaction *oldact) {
  return sigaction(signum, act, oldact);
}
