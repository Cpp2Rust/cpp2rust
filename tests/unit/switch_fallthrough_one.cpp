// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic
#include <cassert>

int fallthrough_one(int x) {
  int r = 0;
  switch (x) {
  case 1:
    r += 10;
  case 2:
    r += 20;
    break;
  default:
    r = -1;
    break;
  }
  return r;
}


int main() {
  assert(fallthrough_one(1) == 30);
  assert(fallthrough_one(2) == 20);
  assert(fallthrough_one(99) == -1);
  return 0;
}
