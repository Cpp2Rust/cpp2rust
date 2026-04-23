// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic
#include <cassert>

int default_first(int x) {
  int r = 0;
  switch (x) {
  default:
    r = 7;
    break;
  case 1:
    r = 1;
    break;
  case 2:
    r = 2;
    break;
  }
  return r;
}


int main() {
  assert(default_first(1) == 1);
  assert(default_first(99) == 7);
  return 0;
}
