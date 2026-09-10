#include <cassert>

struct A {
  int a;
  A(int x) : a(x) {}
};

struct B : A {
  B(int x) : A(x) {}
};

struct C : A {
  C(int x) : A(x + 1) {}
};

struct D : B, C {
  D(int x) : B(x), C(x) {}
  int sum() { return B::a + C::a; }
};

int geta(const A &x) { return x.a; }

int main() {
  D d(1);
  assert(d.sum() == 3);
  B &b = d;
  C &c = d;
  assert(geta(b) == 1);
  assert(geta(c) == 2);
  c.a = 5;
  assert(d.sum() == 6);
  assert(&static_cast<A &>(b) != &static_cast<A &>(c));
  return 0;
}
