#include <cassert>

struct V {
  int x;
  friend int get(const V &v) { return v.x; }
  friend bool operator==(const V &a, const V &b) { return a.x == b.x; }
  template <typename T> friend T scaled(const V &v, T k) { return v.x * k; }
};

template <typename T> struct W {
  T x;
  friend T get(const W &w) { return w.x; }
};

struct D {
  int x;
  friend int declared_then_defined(const D &d);
  friend int declared_only(const D &d);
};

int declared_then_defined(const D &d) { return d.x + 1; }

int main() {
  V a{3};
  V b{3};
  V c{4};
  assert(get(a) == 3);
  assert(a == b);
  assert(!(a == c));
  assert(scaled(c, 2) == 8);
  assert(scaled(c, 1.5) == 6.0);

  W<int> wi{5};
  W<long> wl{6};
  assert(get(wi) == 5);
  assert(get(wl) == 6);

  D d{7};
  assert(declared_then_defined(d) == 8);
  return 0;
}
