// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int *foo(int *p) { return &p[5]; }
int main() {
  int *p1 = new int[10];
  for (unsigned i = 0; i < 10; ++i)
    p1[i] = i;
  int out = foo(&p1[1])[3];
  delete[] p1;
  assert(out == 9);
  return 0;
}
