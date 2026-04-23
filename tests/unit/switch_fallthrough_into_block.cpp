// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic
#include <cassert>

int fallthrough_into_block(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r += 1;
    [[fallthrough]];
  case 2: {
    int tmp = r * 10;
    r = tmp + 5;
    break;
  }
  default:
    r = -1;
    break;
  }
  return r;
}


int main() {
  assert(fallthrough_into_block(1) == 15);
  assert(fallthrough_into_block(2) == 5);
  assert(fallthrough_into_block(99) == -1);
  return 0;
}
