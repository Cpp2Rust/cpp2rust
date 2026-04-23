// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int mixed_return_break(int x) {
  int r = -1;
  switch (x) {
  case 0:
    return 100;
  case 1:
    r = 10;
    break;
  case 2:
    return 200;
  default:
    r = 99;
    break;
  }
  return r;
}


int main() {
  assert(mixed_return_break(0) == 100);
  assert(mixed_return_break(1) == 10);
  assert(mixed_return_break(2) == 200);
  assert(mixed_return_break(99) == 99);
  return 0;
}
