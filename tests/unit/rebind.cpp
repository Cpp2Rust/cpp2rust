// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int x = 1;
  int &r = x;
  int y = 10;
  r = y;
  y += 1;
  assert(x == 10);
  return 0;
}
