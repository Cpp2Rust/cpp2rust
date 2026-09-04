// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <cassert>

int main() {
  int x1 = 1, x2 = 2, x3 = 10, x4 = 20;
  int *p1 = &x1, *p2 = &x2;
  int r1 = std::max(x1, x2);
  int r2 = std::min(x3, x4);
  int r3 = std::max(*p1, x2);
  int r4 = std::min(*p2, x3);
  int r5 = std::max(30, 40);
  assert(r1 + r2 + r3 + r4 + r5 == 56);
  return 0;
}
