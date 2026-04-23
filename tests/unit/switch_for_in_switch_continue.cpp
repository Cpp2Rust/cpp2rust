// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int for_in_switch_continue(int n) {
  int r = 0;
  switch (n) {
  case 0:
    for (int i = 0; i < 5; ++i) {
      if (i % 2 == 0) continue;
      r += i;
    }
    break;
  default:
    r = -1;
    break;
  }
  return r;
}


int main() {
  assert(for_in_switch_continue(0) == 4);
  assert(for_in_switch_continue(99) == -1);
  return 0;
}
