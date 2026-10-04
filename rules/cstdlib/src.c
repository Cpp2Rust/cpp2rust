// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <stdlib.h>
#endif

void CPP2RUST_EXPR_RULE(1)() { return abort(); }

void CPP2RUST_EXPR_RULE(2)(void *a0) { return free(a0); }

void *CPP2RUST_EXPR_RULE(3)(size_t a0) { return malloc(a0); }

void *CPP2RUST_EXPR_RULE(4)(void *a0, size_t a1) { return realloc(a0, a1); }

void *CPP2RUST_EXPR_RULE(5)(size_t nmemb, size_t size) {
  return calloc(nmemb, size);
}

char *CPP2RUST_EXPR_RULE(6)(const char *name) { return getenv(name); }

int CPP2RUST_EXPR_RULE(7)(const char *name, const char *value, int overwrite) {
  return setenv(name, value, overwrite);
}

void *CPP2RUST_EXPR_RULE(8)(const void *key, const void *base, size_t nmemb,
                            size_t size,
                            int (*compar)(const void *, const void *)) {
  return bsearch(key, base, nmemb, size, compar);
}

void CPP2RUST_EXPR_RULE(9)(void *base, size_t nmemb, size_t size,
                           int (*compar)(const void *, const void *)) {
  return qsort(base, nmemb, size, compar);
}

char *CPP2RUST_EXPR_RULE(10)(const char *path, char *resolved_path) {
  return realpath(path, resolved_path);
}
