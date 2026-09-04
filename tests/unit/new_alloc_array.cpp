// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cstring>

int main() {
  int *array = new int[100];
  memset(array, 0, sizeof(int) * 100);
  array[99] = -1;
  for (int *p1 = array; *p1 >= 0; ++p1)
    *p1 = 1;
  int out = 0;
  for (int *p1 = array; *p1 >= 0; ++p1)
    out += *p1;
  int *p2 = array;
  delete[] p2;
  assert(out == 99);
  return 0;
}
