#include <cassert>

template <int N> struct S {
  int v;
  int get() const { return v * N; }
};

template <> struct S<2> {
  int v;
  int w;
  int get() const { return v - w; }
};

template <int N> int scale(int x) { return x * N; }

template <> int scale<2>(int x) { return x - 2; }

int main() {
  S<1> s1{5};
  S<2> s2{5, 1};
  S<3> s3{5};
  assert(s1.get() == 5);
  assert(s2.get() == 4);
  assert(s3.get() == 15);

  assert(scale<1>(7) == 7);
  assert(scale<2>(7) == 5);
  assert(scale<3>(7) == 21);
  return 0;
}
