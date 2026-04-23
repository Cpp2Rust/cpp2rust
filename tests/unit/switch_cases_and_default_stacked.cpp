// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// translation-fail
#include <cassert>

int cases_and_default_stacked(int x) {
  int r = 0;
  switch (x) {
  case 1:
  case 2:
  default:
    r = 42;
    break;
  case 3:
    r = 3;
    break;
  }
  return r;
}


int main() {
  assert(cases_and_default_stacked(1) == 42);
  assert(cases_and_default_stacked(2) == 42);
  assert(cases_and_default_stacked(3) == 3);
  assert(cases_and_default_stacked(99) == 42);
  return 0;
}
