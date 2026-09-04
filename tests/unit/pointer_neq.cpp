// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int x = 5;
  const int *p1 = &x;
  const int *p2 = &x;
  assert(!(p1 != p2));
  return 0;
}
