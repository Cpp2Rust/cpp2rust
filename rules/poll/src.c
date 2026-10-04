// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <poll.h>
#endif

typedef struct pollfd CPP2RUST_TYPE_RULE(1);
typedef nfds_t CPP2RUST_TYPE_RULE(2);

int CPP2RUST_EXPR_RULE(1)(struct pollfd *fds, nfds_t nfds, int timeout) {
  return poll(fds, nfds, timeout);
}
