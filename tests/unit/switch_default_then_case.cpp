// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// translation-fail
#include <cassert>

int default_then_case(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r = 1;
    break;
  default:
  case 2:
    r = 77;
    break;
  case 3:
    r = 3;
    break;
  }
  return r;
}


int main() {
  assert(default_then_case(1) == 1);
  assert(default_then_case(2) == 77);
  assert(default_then_case(3) == 3);
  assert(default_then_case(99) == 77);
  return 0;
}
