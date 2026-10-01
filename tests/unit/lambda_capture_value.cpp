#include <assert.h>

struct S {
  int x;
  int y;
};

int read(const int &v) { return v; }

int main() {
  int factor = 3;
  auto scale = [factor](int x) { return x * factor; };
  assert(scale(4) == 12);
  factor = 100;
  assert(scale(4) == 12);

  int slot = 7;
  int *p = &slot;
  auto read_ptr = [p]() { return *p; };
  slot = 8;
  assert(read_ptr() == 8);

  S s = {1, 2};
  auto sum = [s]() { return s.x + s.y; };
  s.x = 50;
  assert(sum() == 3);

  int base = 10;
  auto shifted = [y = base + 1](int x) { return x + y; };
  assert(shifted(5) == 16);

  const int k = 3;
  auto by_copy = [k](int x) { return x + k; };
  assert(by_copy(1) == 4);

  auto by_copy_used = [k](int x) { return x + read(k); };
  assert(by_copy_used(1) == 4);

  auto implicit_used = [=](int x) { return x + read(k); };
  assert(implicit_used(1) == 4);

  auto by_ref = [&k](int x) { return x + k; };
  assert(by_ref(1) == 4);

  return 0;
}
