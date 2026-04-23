// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int switch_char(char c) {
  switch (c) {
  case 'a':
    return 1;
  case 'b':
    return 2;
  case '\n':
    return 3;
  case '\0':
    return 4;
  default:
    return 0;
  }
}

enum Color { kRed, kGreen, kBlue };

int main() {
  assert(switch_char('a') == 1);
  assert(switch_char('b') == 2);
  assert(switch_char('\n') == 3);
  assert(switch_char('\0') == 4);
  assert(switch_char('z') == 0);
  return 0;
}
