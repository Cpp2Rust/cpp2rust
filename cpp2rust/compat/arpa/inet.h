// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include_next <arpa/inet.h>

#undef ntohl
#undef ntohs
#undef htonl
#undef htons

uint32_t cpp2rust_ntohl(uint32_t x);
uint16_t cpp2rust_ntohs(uint16_t x);
uint32_t cpp2rust_htonl(uint32_t x);
uint16_t cpp2rust_htons(uint16_t x);

#define ntohl(x) cpp2rust_ntohl(x)
#define ntohs(x) cpp2rust_ntohs(x)
#define htonl(x) cpp2rust_htonl(x)
#define htons(x) cpp2rust_htons(x)
