// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int stacked_block(int x) {
  int r = 0;
  switch (x) {
  case 1:
  case 2:
  case 3: {
    int y = x * 2;
    r = y + 1;
    break;
  }
  default:
    r = 0;
    break;
  }
  return r;
}

int main() {
  assert(stacked_block(2) == 5);
  assert(stacked_block(9) == 0);
  return 0;
}
