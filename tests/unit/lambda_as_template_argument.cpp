#include <assert.h>

template <typename F> int apply(F fn, int x) { return fn(x); }

template <typename F> int apply_twice(F fn, int x) { return fn(fn(x)); }

int main() {
  int base = 10;
  auto add_base = [&base](int x) { return x + base; };
  assert(apply(add_base, 5) == 15);
  base = 100;
  assert(apply(add_base, 5) == 105);

  int factor = 3;
  auto scale = [factor](int x) { return x * factor; };
  assert(apply(scale, 4) == 12);
  assert(apply_twice(scale, 4) == 36);

  assert(apply([](int x) { return -x; }, 9) == -9);

  return 0;
}
