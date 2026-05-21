// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <poll.h>

int f1(struct pollfd *fds, nfds_t nfds, int timeout) {
  return poll(fds, nfds, timeout);
}
