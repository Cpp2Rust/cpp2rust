// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct POD {
  int x1, x2, x3;
};

void PODIncrement(POD &pod) {
  pod.x1 += 1;
  pod.x2 += 2;
  pod.x3 += 3;
}

int main() {
  POD p1 = {10, 11, 12};
  POD p2 = {p1.x1, p1.x2, p1.x3};
  PODIncrement(p2);
  assert(p2.x1 + p2.x2 + p2.x3 == 39);
  return 0;
}
