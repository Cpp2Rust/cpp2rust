// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int basic(int x) {
  int r = 0;
  switch (x) {
  case 0:
    r = 10;
    break;
  case 1:
    r = 20;
    break;
  case 2:
    r = 30;
    break;
  default:
    r = 40;
    break;
  }
  return r;
}

int main() {
  assert(basic(0) == 10);
  assert(basic(2) == 30);
  assert(basic(99) == 40);
  return 0;
}
