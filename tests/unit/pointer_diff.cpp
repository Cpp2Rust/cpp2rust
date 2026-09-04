// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int a[] = {1, 2, 3, 4, 5};
  const int *p0 = &a[0];
  const int *p1 = &a[4];
  assert(p1 - p0 == 4);
  return 0;
}
