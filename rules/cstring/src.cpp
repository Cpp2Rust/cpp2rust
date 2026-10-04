// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <string.h>
#endif

const char *CPP2RUST_EXPR_RULE(6)(const char *a0, int a1) {
  return strchr(a0, a1);
}

const void *CPP2RUST_EXPR_RULE(12)(const void *a0, int a1, size_t a2) {
  return memchr(a0, a1, a2);
}

const char *CPP2RUST_EXPR_RULE(13)(const char *a0, int a1) {
  return strrchr(a0, a1);
}

char *CPP2RUST_EXPR_RULE(14)(char *a0, int a1) { return strrchr(a0, a1); }

const char *CPP2RUST_EXPR_RULE(19)(const char *a0, const char *a1) {
  return strstr(a0, a1);
}

char *CPP2RUST_EXPR_RULE(20)(char *a0, const char *a1) {
  return strstr(a0, a1);
}

const char *CPP2RUST_EXPR_RULE(22)(const char *a0, const char *a1) {
  return strpbrk(a0, a1);
}

char *CPP2RUST_EXPR_RULE(23)(char *a0, const char *a1) {
  return strpbrk(a0, a1);
}

#if defined(__linux__)
const void *CPP2RUST_EXPR_RULE(25)(const void *a0, int a1, size_t a2) {
  return memrchr(a0, a1, a2);
}

void *CPP2RUST_EXPR_RULE(26)(void *a0, int a1, size_t a2) {
  return memrchr(a0, a1, a2);
}
#endif
