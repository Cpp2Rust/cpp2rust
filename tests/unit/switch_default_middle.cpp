// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic
#include <cassert>

int default_middle(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r = 1;
    break;
  default:
    r = 99;
    break;
  case 2:
    r = 2;
    break;
  }
  return r;
}

int main() {
  assert(default_middle(1) == 1);
  assert(default_middle(2) == 2);
  assert(default_middle(99) == 99);
  return 0;
}
