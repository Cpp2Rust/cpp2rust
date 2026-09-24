// translation-fail
#include <assert.h>

template <typename F> int apply_int(F fn, int x) { return fn(x); }

template <typename F> double apply_double(F fn, double x) { return fn(x); }

template <typename F> int apply_twice(F fn, int x) { return fn(fn(x)); }

int main() {
  int factor = 3;
  auto scale = [factor](int x) { return x * factor; };
  assert(apply_twice(scale, 4) == 36);

  assert(apply_int([](int x) { return -x; }, 9) == -9);

  auto generic_scale = [factor](auto x) { return x * factor; };
  assert(apply_int(generic_scale, 4) == 12);
  assert(apply_double(generic_scale, 1.5) == 4.5);

  assert(apply_int([](auto x) { return -x; }, 9) == -9);

  auto offset = [factor]<typename T>(T x) { return x + factor; };
  assert(apply_int(offset, 4) == 7);
  assert(apply_double(offset, 1.5) == 4.5);

  return 0;
}
