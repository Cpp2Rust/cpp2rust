// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int h = 15;

  int &h_ref1 = h;
  h_ref1 = 16;

  int *h_ptr = &h_ref1;
  int &h_ref2 = *h_ptr;
  h_ref2 = 17;

  assert(h_ref1 + h_ref2 == 34);
  return 0;
}
