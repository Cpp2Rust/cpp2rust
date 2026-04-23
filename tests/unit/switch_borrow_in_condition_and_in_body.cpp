// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic
#include <cassert>

int borrow_in_condition_and_in_body(int x) {
  switch (x) {
  case 0:
    [[fallthrough]];
  default:
    return x + 1;
  }
}


int main() {
  assert(borrow_in_condition_and_in_body(0) == 1);
  assert(borrow_in_condition_and_in_body(1) == 2);
  return 0;
}
