// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <netdb.h>
#endif

typedef struct addrinfo CPP2RUST_TYPE_RULE(1);

int CPP2RUST_EXPR_RULE(1)(const char *node, const char *service,
                          const struct addrinfo *hints, struct addrinfo **res) {
  return getaddrinfo(node, service, hints, res);
}

void CPP2RUST_EXPR_RULE(2)(struct addrinfo *res) { return freeaddrinfo(res); }
