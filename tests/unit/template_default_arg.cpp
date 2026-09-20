#include <cassert>

struct A {
  int v;
  A() : v(1) {}
  A(int v) : v(v) {}
};

struct B {
  int v;
  B() : v(2) {}
};

struct NoDefault {
  int v;
  NoDefault(int v) : v(v) {}
};

template <typename T> int used(T x = T()) { return x.v; }

template <typename T> int scaled(T x, int n = sizeof(T)) { return x.v * n; }

template <typename T> int always_given(T x = T()) { return x.v; }

template <typename T> struct S {
  int v;
  S(int v) : v(v) {}
  int get(T t = T()) const { return v + t.v; }
};

int main() {
  assert(used<A>() == 1);
  assert(used(A(5)) == 5);
  assert(used<B>() == 2);
  assert(scaled(A(3)) == 3 * (int)sizeof(A));
  assert(scaled(A(3), 2) == 6);

  assert(always_given(NoDefault(3)) == 3);
  S<NoDefault> s(1);
  assert(s.get(NoDefault(4)) == 5);
  return 0;
}
