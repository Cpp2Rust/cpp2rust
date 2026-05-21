// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <arpa/inet.h>

uint32_t f1(uint32_t x) { return ntohl(x); }
uint16_t f2(uint16_t x) { return ntohs(x); }
uint16_t f3(uint16_t x) { return htons(x); }
uint32_t f4(uint32_t x) { return htonl(x); }
