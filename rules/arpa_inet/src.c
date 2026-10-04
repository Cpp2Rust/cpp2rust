// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <arpa/inet.h>
#endif

uint32_t CPP2RUST_EXPR_RULE(1)(uint32_t x) { return ntohl(x); }
uint16_t CPP2RUST_EXPR_RULE(2)(uint16_t x) { return ntohs(x); }
uint16_t CPP2RUST_EXPR_RULE(3)(uint16_t x) { return htons(x); }
uint32_t CPP2RUST_EXPR_RULE(4)(uint32_t x) { return htonl(x); }
int CPP2RUST_EXPR_RULE(5)(int af, const char *src, void *dst) {
  return inet_pton(af, src, dst);
}
const char *CPP2RUST_EXPR_RULE(6)(int af, const void *src, char *dst,
                                  socklen_t size) {
  return inet_ntop(af, src, dst, size);
}
