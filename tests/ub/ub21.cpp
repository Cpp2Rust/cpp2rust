// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
#include <cstddef>
size_t strlen(const char *s) {
  size_t count = 0;
  while (*s++)
    ++count;
  return count;
}

int main() {
  const char s[] = {'s', 't', 'r'};
  return strlen(s);
}
