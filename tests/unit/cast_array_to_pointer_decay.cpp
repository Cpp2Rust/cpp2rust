// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int deref(int *p) { return *p; }
int strlen(char *s) {
  int c = 0;
  while (*s++)
    ++c;
  return c;
}
int main() {
  int a[2] = {1, 2};
  char s[4] = {'a', 'b', 'c', '\0'};
  assert(deref(a) + strlen(s) == 4);
  return 0;
}
