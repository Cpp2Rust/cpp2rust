// translation-fail
#include <assert.h>

int main() {
  auto twice = [](auto x) { return x + x; };
  assert(twice(4) == 8);
  assert(twice(1.5) == 3.0);

  int base = 10;
  auto add_base = [base](auto x) { return x + base; };
  assert(add_base(5) == 15);
  assert(add_base(2.5) == 12.5);

  int total = 0;
  auto accumulate = [&total](auto x, auto y) { total += x * y; };
  accumulate(2, 3);
  accumulate(4u, 5u);
  assert(total == 26);

  auto sub = []<typename T>(T x, T y) { return x - y; };
  assert(sub(9, 4) == 5);
  assert(sub(2.5, 1.0) == 1.5);

  auto mixed = [base]<typename T, typename U>(T x, U y) {
    return x * y + base;
  };
  assert(mixed(2, 3) == 16);
  assert(mixed(2, 0.5) == 11.0);

  return 0;
}
