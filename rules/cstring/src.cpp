// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#define _GNU_SOURCE
#include <string.h>

const char *f6(const char *a0, int a1) { return strchr(a0, a1); }

const void *f12(const void *a0, int a1, size_t a2) { return memchr(a0, a1, a2); }

const char *f13(const char *a0, int a1) { return strrchr(a0, a1); }

char *f14(char *a0, int a1) { return strrchr(a0, a1); }

const char *f19(const char *a0, const char *a1) { return strstr(a0, a1); }

char *f20(char *a0, const char *a1) { return strstr(a0, a1); }

const char *f22(const char *a0, const char *a1) { return strpbrk(a0, a1); }

char *f23(char *a0, const char *a1) { return strpbrk(a0, a1); }

const void *f25(const void *a0, int a1, size_t a2) { return memrchr(a0, a1, a2); }

void *f26(void *a0, int a1, size_t a2) { return memrchr(a0, a1, a2); }
