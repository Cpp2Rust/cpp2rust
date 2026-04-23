// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int continue_inside_switch(int n) {
  int r = 0;
  for (int i = 0; i < n; ++i) {
    switch (i) {
    case 0:
    case 2:
    case 4:
      continue;
    default:
      r += i;
      break;
    }
    r += 1000;
  }
  return r;
}

int main() {
  assert(continue_inside_switch(6) == (1 + 3 + 5) + 3 * 1000);
  return 0;
}
