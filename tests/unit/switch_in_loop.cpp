// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int switch_in_loop(int n) {
  int r = 0;
  for (int i = 0; i < n; ++i) {
    switch (i % 3) {
    case 0:
      r += 1;
      break;
    case 1:
      r += 2;
      break;
    default:
      r += 3;
      break;
    }
    r += 10;
  }
  return r;
}


int main() {
  assert(switch_in_loop(6) == 72);
  return 0;
}
