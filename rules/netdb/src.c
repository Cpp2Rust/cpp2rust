// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#undef _GNU_SOURCE
#define _GNU_SOURCE
#include <netdb.h>

typedef struct addrinfo t1;

int f1(const char *node, const char *service, const struct addrinfo *hints,
       struct addrinfo **res) {
  return getaddrinfo(node, service, hints, res);
}

void f2(struct addrinfo *res) { return freeaddrinfo(res); }
