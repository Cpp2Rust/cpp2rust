// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int strlen(const char *s, int n) { return *s ? strlen(s + 1, n + 1) : n; }

int main() {
  const char s[] = {'s', 't', 'r', '\0'};
  assert(strlen(&s[0], 0) == 3);
  return 0;
}
