// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <sys/select.h>
#endif

typedef fd_set CPP2RUST_TYPE_RULE(1);

int CPP2RUST_EXPR_RULE(1)(int nfds, fd_set *readfds, fd_set *writefds,
                          fd_set *exceptfds, struct timeval *timeout) {
  return select(nfds, readfds, writefds, exceptfds, timeout);
}

void CPP2RUST_EXPR_RULE(2)(int fd, fd_set *set) { return FD_SET(fd, set); }
void CPP2RUST_EXPR_RULE(3)(int fd, fd_set *set) { return FD_CLR(fd, set); }
int CPP2RUST_EXPR_RULE(4)(int fd, const fd_set *set) {
  return FD_ISSET(fd, set);
}
void CPP2RUST_EXPR_RULE(5)(fd_set *set) { return FD_ZERO(set); }
