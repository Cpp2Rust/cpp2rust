// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <stdlib.h>

void f1(void) { return abort(); }

void f2(void *a0) { return free(a0); }

void *f3(size_t a0) { return malloc(a0); }

void *f4(void *a0, size_t a1) { return realloc(a0, a1); }

void *f5(size_t nmemb, size_t size) { return calloc(nmemb, size); }

char *f6(const char *name) { return getenv(name); }

void *f7(const void *key, const void *base, size_t nmemb, size_t size,
         int (*compar)(const void *, const void *)) {
  return bsearch(key, base, nmemb, size, compar);
}

void f8(void *base, size_t nmemb, size_t size,
        int (*compar)(const void *, const void *)) {
  return qsort(base, nmemb, size, compar);
}

char *f9(const char *path, char *resolved_path) {
  return realpath(path, resolved_path);
}

void f10(int a0) { return exit(a0); }
