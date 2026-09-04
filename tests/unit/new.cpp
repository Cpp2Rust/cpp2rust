// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int *x = new int(5);
  int out = *x;
  delete x;
  assert(out == 5);
  return 0;
}
