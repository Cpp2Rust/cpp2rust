// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic
#include <cassert>

int for_switch_for_break(int n) {
  int r = 0;
  for (int i = 0; i < n; ++i) {
    switch (i) {
    case 1:
      for (int j = 0; j < 10; ++j) {
        if (j == 2)
          break;
        r += 1;
      }
      r += 100;
      break;
    default:
      r += 10;
      break;
    }
  }
  return r;
}

int main() {
  assert(for_switch_for_break(3) == 122);
  return 0;
}
