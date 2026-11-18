// ADDITIONAL_COMPILE_FLAGS: -std=c++20

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

template <float V> float get_float() { return V; }

template <double V> double get_double() { return V; }

template <long double V> long double get_long_double() { return V; }

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

  float f_nan = get_float<__builtin_nanf("")>();
  float f_inf = get_float<-__builtin_inff()>();
  assert(f_nan != f_nan);
  assert(f_inf < 0);
  double d_nan = get_double<__builtin_nan("0x5")>();
  double d_inf = get_double<-__builtin_inf()>();
  assert(d_nan != d_nan);
  assert(d_inf < 0);
  long double ld_nan = get_long_double<__builtin_nanl("")>();
  long double ld_inf = get_long_double<-__builtin_infl()>();
  assert(ld_nan != ld_nan);
  assert(ld_inf < 0);
  return 0;
}
