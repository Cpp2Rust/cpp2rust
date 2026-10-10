#include <assert.h>

int main() {
  auto zero = []() { return 42; };
  assert(zero() == 42);

  auto one = [](int x) { return x + 1; };
  assert(one(1) == 2);

  auto three = [](int x, int y, int z) { return x * 100 + y * 10 + z; };
  assert(three(1, 2, 3) == 123);

  const int k = 3;
  constexpr int m = 4;
  auto constants = [](int x) { return x + k + m; };
  assert(constants(1) == 8);

  const int n = k + m;
  auto derived = [](int x) { return x + n; };
  assert(derived(1) == 8);

  auto implicit = [=](int x) { return x + k; };
  assert(implicit(1) == 4);

  auto bump = [](int x) {
    x += 1;
    return x;
  };
  assert(bump(1) == 2);

  auto through_ptr = [](int x) {
    int *p = &x;
    *p = *p * 2;
    return x;
  };
  assert(through_ptr(4) == 8);

  int seed = 5;
  int boxed = through_ptr(seed);
  int *boxed_ptr = &boxed;
  assert(*boxed_ptr == 10);
  assert(seed == 5);

  return 0;
}
