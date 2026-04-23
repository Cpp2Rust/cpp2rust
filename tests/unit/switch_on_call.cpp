// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int double_it(int v) { return v * 2; }

int switch_on_call(int x) {
  switch (double_it(x)) {
  case 0:
    return 100;
  case 2:
    return 200;
  case 4:
    return 400;
  default:
    return -1;
  }
}


int main() {
  assert(switch_on_call(0) == 100);
  assert(switch_on_call(1) == 200);
  assert(switch_on_call(2) == 400);
  assert(switch_on_call(99) == -1);
  return 0;
}
