// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int x = 1;
  int &r = x;
  r = 5;
  assert(x == 5);
  return 0;
}
