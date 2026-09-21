// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <cassert>
#include <compare>

struct Defaulted {
  int a;
  int b;
  friend bool operator==(const Defaulted &, const Defaulted &) = default;
};

struct DefaultedOrd {
  int a;
  friend auto operator<=>(const DefaultedOrd &, const DefaultedOrd &) = default;
  friend bool operator==(const DefaultedOrd &, const DefaultedOrd &) = default;
};

struct Inline {
  int a;
  friend bool operator==(const Inline &x, const Inline &y) {
    return x.a == y.a;
  }
  friend bool operator<(const Inline &x, const Inline &y) { return x.a < y.a; }
  friend Inline operator+(const Inline &x, const Inline &y) {
    return Inline{x.a + y.a};
  }
};

struct OutOfLine {
  int a;
  friend bool operator==(const OutOfLine &x, const OutOfLine &y);
  friend bool operator!=(const OutOfLine &x, const OutOfLine &y);
};

bool operator==(const OutOfLine &x, const OutOfLine &y) { return x.a == y.a; }
bool operator!=(const OutOfLine &x, const OutOfLine &y) { return !(x == y); }

template <class T> struct Tmpl {
  T v;
  friend bool operator==(const Tmpl &x, const Tmpl &y) { return x.v == y.v; }
  friend bool operator<(const Tmpl &x, const Tmpl &y) { return x.v < y.v; }
  template <class U> friend bool operator==(const Tmpl &x, const Tmpl<U> &y) {
    return x.v == y.v;
  }
};

template <class T> struct TmplDefaulted {
  T v;
  friend bool operator==(const TmplDefaulted &,
                         const TmplDefaulted &) = default;
};

int main() {
  Defaulted d1{1, 2}, d2{1, 2}, d3{1, 3};
  assert(d1 == d2);
  assert(d1 != d3);
  assert(!(d1 == d3));

  DefaultedOrd o1{1}, o2{2};
  assert(o1 < o2);
  assert(o2 > o1);
  assert(o1 == o1);
  assert((o1 <=> o2) == std::strong_ordering::less);

  Inline i1{1}, i2{2}, i3{1};
  assert(i1 == i3);
  assert(!(i1 == i2));
  assert(i1 < i2);
  assert(!(i2 < i1));
  assert(i1 + i2 == Inline{3});

  OutOfLine f1{4}, f2{4}, f3{5};
  assert(f1 == f2);
  assert(f1 != f3);
  assert(!(f1 == f3));

  Tmpl<int> t1{1}, t2{2}, t3{1};
  assert(t1 == t3);
  assert(!(t1 == t2));
  assert(t1 < t2);
  Tmpl<long> u1{1}, u2{2};
  assert(t1 == u1);
  assert(!(t1 == u2));

  TmplDefaulted<int> v1{7}, v2{7}, v3{8};
  assert(v1 == v2);
  assert(v1 != v3);
  assert(!(v1 == v3));

  return 0;
}
