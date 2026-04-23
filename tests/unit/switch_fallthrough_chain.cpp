// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic
#include <cassert>

int fallthrough_chain(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r += 1;
    [[fallthrough]];
  case 2:
    r += 2;
    [[fallthrough]];
  case 3:
    r += 4;
    [[fallthrough]];
  case 4:
    r += 8;
    break;
  default:
    r = -1;
    break;
  }
  return r;
}

int main() {
  assert(fallthrough_chain(1) == 15);
  assert(fallthrough_chain(2) == 14);
  assert(fallthrough_chain(3) == 12);
  assert(fallthrough_chain(4) == 8);
  assert(fallthrough_chain(99) == -1);
  return 0;
}
