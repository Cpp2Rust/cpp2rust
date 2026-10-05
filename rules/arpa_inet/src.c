// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <arpa/inet.h>

uint32_t f1(uint32_t x) { return ntohl(x); }
uint16_t f2(uint16_t x) { return ntohs(x); }
uint16_t f3(uint16_t x) { return htons(x); }
uint32_t f4(uint32_t x) { return htonl(x); }
int f5(int af, const char *src, void *dst) { return inet_pton(af, src, dst); }
const char *f6(int af, const void *src, char *dst, socklen_t size) {
  return inet_ntop(af, src, dst, size);
}
