// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

void more_refs(int x1, int x2, int &r1, const int &r2) {
  const int &rx1 = x1;
  int &rx2 = x2;
  const int *pr1 = &r1;
  const int *pr2 = &r2;
  const int &rpr1 = *pr1;
  const int &rpr2 = *pr2;
  const int &r = r1;
  rx2 += 1 + rx1 + rx2 + *pr1 + *pr2 + rpr1 + rpr2 + r;
  r1 = rx2;
}

int main() {
  int x1 = 1;
  const int x2 = 2;
  more_refs(3, 4, x1, x2);
  assert(x1 + x2 == 21);
  return 0;
}
