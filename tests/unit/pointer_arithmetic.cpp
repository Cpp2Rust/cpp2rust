// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int x = 1;
  int *p = &x;
  *p += 1;
  assert(x == 2);
  int a[] = {1, 2};
  p = &a[1];
  *p += 1;
  assert(a[0] == 1 && a[1] == 3);
  --p;
  *p += 1;
  assert(a[0] == 2 && a[1] == 3);
  p = &x;
  *p += 1;
  assert(x == 3);
  int *p2 = p;
  *p2 += 1;
  assert(x == 4);
  return 0;
}
