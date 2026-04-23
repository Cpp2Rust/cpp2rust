// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int switch_complex_cond(int *p, int bias) {
  switch (*p + bias) {
  case 0:
    return 1;
  case 5:
    return 2;
  case 10:
    return 3;
  default:
    return 0;
  }
}


int main() {
  int p_val = 5;
  assert(switch_complex_cond(&p_val, 0) == 2);
  assert(switch_complex_cond(&p_val, 5) == 3);
  assert(switch_complex_cond(&p_val, -5) == 1);
  assert(switch_complex_cond(&p_val, 99) == 0);
  return 0;
}
