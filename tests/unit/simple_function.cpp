// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int foo() { return 0; }

int main() {
  int x = foo() + 1;
  int y = foo();
  assert(x + y == 1);
  return 0;
}
