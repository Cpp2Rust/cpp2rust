#include <cassert>

struct A {
  int a;
  A(int x) : a(x) {}
};

struct B {
  int b;
  B(int x) : b(x) {}
};

struct C : A, B {
  int c;
  C(int x) : A(x), B(x + 1), c(x + 2) {}
  int sum() { return a + b + c; }
};

int geta(const A &x) { return x.a; }
int getb(B *x) { return x->b; }

int main() {
  C c(1);
  assert(c.sum() == 6);
  assert(geta(c) == 1);
  assert(getb(&c) == 2);
  B *pb = &c;
  pb->b = 10;
  assert(c.b == 10);
  assert(c.sum() == 14);
  return 0;
}
