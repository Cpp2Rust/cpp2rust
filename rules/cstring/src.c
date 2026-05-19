// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#define _GNU_SOURCE
#include <string.h>
#include <strings.h>

void *f1(void *dst, const void *src, size_t n) { return memcpy(dst, src, n); }

void *f2(void *dst, int c, size_t n) { return memset(dst, c, n); }

int f3(const void *s1, const void *s2, size_t n) { return memcmp(s1, s2, n); }

void *f4(void *dst, const void *src, size_t n) { return memmove(dst, src, n); }

char *f5(const char *a0, int a1) { return strchr(a0, a1); }

size_t f7(const char *a0) { return strlen(a0); }

int f8(const char *a0, const char *a1) { return strcmp(a0, a1); }

int f9(const char *a0, const char *a1, size_t a2) { return strncmp(a0, a1, a2); }

void *f10(const void *a0, int a1, size_t a2) { return memchr(a0, a1, a2); }

char *f11(const char *a0, int a1) { return strrchr(a0, a1); }

char *f15(const char *a0) { return strdup(a0); }

size_t f16(const char *a0, const char *a1) { return strcspn(a0, a1); }

size_t f17(const char *a0, const char *a1) { return strspn(a0, a1); }

char *f18(const char *a0, const char *a1) { return strstr(a0, a1); }

char *f21(const char *a0, const char *a1) { return strpbrk(a0, a1); }

void *f24(const void *a0, int a1, size_t a2) { return memrchr(a0, a1, a2); }

int f27(const char *a0, const char *a1) { return strcasecmp(a0, a1); }
