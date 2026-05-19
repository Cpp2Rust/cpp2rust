// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <netdb.h>

int f1(const char *node, const char *service, const struct addrinfo *hints,
       struct addrinfo **res) {
  return getaddrinfo(node, service, hints, res);
}

void f2(struct addrinfo *res) { return freeaddrinfo(res); }
