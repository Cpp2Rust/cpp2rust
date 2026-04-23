// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int empty_case_with_break(int x) {
  int r = 5;
  switch (x) {
  case 1:
    break;
  case 2:
    r = 2;
    break;
  default:
    r = 9;
    break;
  }
  return r;
}


int main() {
  assert(empty_case_with_break(1) == 5);
  assert(empty_case_with_break(2) == 2);
  assert(empty_case_with_break(9) == 9);
  return 0;
}
