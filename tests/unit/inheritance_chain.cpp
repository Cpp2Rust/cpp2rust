#include <cassert>

struct A {
  int a;
};

struct B : A {
  int b;
  B(int x) : b(x + 1) { a = x; }
};

struct C : B {
  using B::B;
  int sum() { return a + b; }
};

int geta(const A &x) { return x.a; }

int main() {
  C c(1);
  assert(c.sum() == 3);
  assert(geta(c) == 1);
  return 0;
}
