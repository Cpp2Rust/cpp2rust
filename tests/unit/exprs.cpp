// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct X {
  int x;
};

struct Y {
  X x;
  X &foo() { return x; }
  X *p;
  X *ptr() { return &x; }
};

int main() {
  int x1 = 5;
  int x2 = x1;
  int x3 = x1 + 5;
  int x4 = x3 + x2;
  x1 = 5;
  x2 = x1;
  x3 = x1 + 5;
  x4 = x3 + x2;
  int *p1 = &x1;
  p1 = &x2;
  *p1 = x1;
  *p1 = x1 + x4 + 1;
  [[maybe_unused]] int x5 = *p1;
  [[maybe_unused]] int x6 = *p1 + x3 + 5;
  int &r = x1;
  r = 5;
  r = *p1 + 5;
  [[maybe_unused]] int x7 = r;
  [[maybe_unused]] int x8 = r + x1 + 5;
  [[maybe_unused]] int *p2 = &r;
  X x{1};
  Y y = {{0}, &x};
  y.x.x = 5;
  y.foo().x = 1;
  y.p->x = 10;
  Y *p3 = &y;
  p3->p->x = 100;
  y.ptr()->x = 1;
  (*y.ptr()).x = 50;
  assert(x.x == 100);
  return 0;
}
