// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <string.h>
#include <strings.h>
#endif

void *CPP2RUST_EXPR_RULE(1)(void *dst, const void *src, size_t n) {
  return memcpy(dst, src, n);
}

void *CPP2RUST_EXPR_RULE(2)(void *dst, int c, size_t n) {
  return memset(dst, c, n);
}

int CPP2RUST_EXPR_RULE(3)(const void *s1, const void *s2, size_t n) {
  return memcmp(s1, s2, n);
}

void *CPP2RUST_EXPR_RULE(4)(void *dst, const void *src, size_t n) {
  return memmove(dst, src, n);
}

char *CPP2RUST_EXPR_RULE(5)(const char *a0, int a1) { return (strchr)(a0, a1); }

size_t CPP2RUST_EXPR_RULE(7)(const char *a0) { return strlen(a0); }

int CPP2RUST_EXPR_RULE(8)(const char *a0, const char *a1) {
  return strcmp(a0, a1);
}

int CPP2RUST_EXPR_RULE(9)(const char *a0, const char *a1, size_t a2) {
  return strncmp(a0, a1, a2);
}

void *CPP2RUST_EXPR_RULE(10)(const void *a0, int a1, size_t a2) {
  return (memchr)(a0, a1, a2);
}

char *CPP2RUST_EXPR_RULE(11)(const char *a0, int a1) {
  return (strrchr)(a0, a1);
}

char *CPP2RUST_EXPR_RULE(15)(const char *a0) { return strdup(a0); }

size_t CPP2RUST_EXPR_RULE(16)(const char *a0, const char *a1) {
  return strcspn(a0, a1);
}

size_t CPP2RUST_EXPR_RULE(17)(const char *a0, const char *a1) {
  return strspn(a0, a1);
}

char *CPP2RUST_EXPR_RULE(18)(const char *a0, const char *a1) {
  return (strstr)(a0, a1);
}

char *CPP2RUST_EXPR_RULE(21)(const char *a0, const char *a1) {
  return (strpbrk)(a0, a1);
}

#if defined(__linux__)
void *CPP2RUST_EXPR_RULE(24)(const void *a0, int a1, size_t a2) {
  return memrchr(a0, a1, a2);
}
#endif

int CPP2RUST_EXPR_RULE(27)(const char *a0, const char *a1) {
  return strcasecmp(a0, a1);
}

#if defined(__linux__)
char *CPP2RUST_EXPR_RULE(28)(int errnum, char *buf, size_t buflen) {
  return strerror_r(errnum, buf, buflen);
}
#elif defined(__APPLE__)
int CPP2RUST_EXPR_RULE(28)(int errnum, char *buf, size_t buflen) {
  return strerror_r(errnum, buf, buflen);
}
#else
#error "Unsupported platform for strerror_r"
#endif
