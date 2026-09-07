// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int foo(int x) { return x; }
int foo(int *x) { return *x; }
int foo(int *x, int *y) { return *x + *y; }
int foo(int *x, int *y, int &z) { return *x + *y + z; }
int bar(int &x) { return x; }

class Foo {
public:
  void foo() const {}
  void foo() {}
  void method(int x) {}
  void method(int x) const {}
  void method2(int x, int y) const {}
  void method2(double x, double y) const {}
};

namespace ns1 {
int func(int x) { return 1; }
int func(int *x) { return 1; }
} // namespace ns1

int main() {
  int x = 1;
  int out = 0;
  out += foo(0);
  out += foo(&x);
  out += bar(x);
  out += foo(&x, &x, x);
  out += foo(&x, &x);
  int bar = 5;
  out += bar + foo(0) + foo(&x);
  Foo foo1{};
  const Foo foo2{};
  foo1.foo();
  foo1.method(1);
  foo2.foo();
  foo2.method(2);
  assert(out == 13);
  return 0;
}
