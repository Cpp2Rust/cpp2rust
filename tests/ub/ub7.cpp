// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
#include <cstddef>

size_t strlen(const char *s) {
  const char *begin = s;
  while (*s) {
    ++s;
  }
  return s - begin;
}

int main() {
  const char s[] = {'s', 't', 'r', 'i', 'n', 'g'};
  return strlen(&s[0]);
}
