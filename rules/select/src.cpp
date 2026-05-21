// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/select.h>

int f1(int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds,
       struct timeval *timeout) {
  return select(nfds, readfds, writefds, exceptfds, timeout);
}
