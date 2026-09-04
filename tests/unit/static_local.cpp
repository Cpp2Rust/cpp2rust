// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int foo() {
  static int static_i;
  static float static_f;
  static bool static_b;

  static int kX1 = 1;
  static const int kX2 = 2;
  kX1 += 1;
  return kX1 + kX2 + static_i;
}

int main() {
  assert(foo() + foo() + foo() == 15);
  return 0;
}
