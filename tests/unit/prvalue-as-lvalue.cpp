// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

const int &foo(const int &a) { return a; }

int main() {
  int a = 1, *pa = &a;
  const int &b = foo(*pa);
  assert(b == 1);
  return 0;
}
