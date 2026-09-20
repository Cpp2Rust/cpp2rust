#include <cassert>

template <typename T> T foo(T x) { return x; }

template <typename T> T *bar(T *p, bool flag) { return flag ? p : nullptr; }

template <typename T1, typename T2, typename T3> int func(T1 x1, T2 x2, T3 x3) {
  return x1 + x2 + x3;
}

template <typename T> T half = T(1) / T(2);

int main() {
  int x = 10;
  double y = x;
  assert(foo(x) + foo(y) + *bar(&x, true) + *bar(&y, true) + func(1, 2, 3) +
             func(2.0, x, y) ==
         68);
  assert(half<int> == 0);
  assert(half<double> == 0.5);
  half<int> = 7;
  assert(half<int> == 7);
  assert(half<double> == 0.5);
  return 0;
}
