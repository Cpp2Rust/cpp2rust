// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/select.h>

using t1 = fd_set;

int f1(int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds,
       struct timeval *timeout) {
  return select(nfds, readfds, writefds, exceptfds, timeout);
}

void f2(int fd, fd_set *set) { return FD_SET(fd, set); }
void f3(int fd, fd_set *set) { return FD_CLR(fd, set); }
int f4(int fd, const fd_set *set) { return FD_ISSET(fd, set); }
void f5(fd_set *set) { return FD_ZERO(set); }
