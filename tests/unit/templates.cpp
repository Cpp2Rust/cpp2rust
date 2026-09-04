// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

template <typename T> T foo(T x) { return x; }

template <typename T> T *bar(T *p, bool flag) { return flag ? p : nullptr; }

template <typename T1, typename T2, typename T3> int func(T1 x1, T2 x2, T3 x3) {
  return x1 + x2 + x3;
}

int main() {
  int x = 10;
  double y = x;
  assert(foo(x) + foo(y) + *bar(&x, true) + *bar(&y, true) + func(1, 2, 3) +
             func(2.0, x, y) ==
         68);
  return 0;
}
