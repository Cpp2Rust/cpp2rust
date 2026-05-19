// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <string.h>

void *f1(void *dst, const void *src, size_t n) { return memcpy(dst, src, n); }

void *f2(void *dst, int c, size_t n) { return memset(dst, c, n); }

int f3(const void *s1, const void *s2, size_t n) { return memcmp(s1, s2, n); }

void *f4(void *dst, const void *src, size_t n) { return memmove(dst, src, n); }

char *f5(const char *a0, int a1) { return strchr(a0, a1); }

size_t f7(const char *a0) { return strlen(a0); }
