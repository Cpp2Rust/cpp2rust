#include <assert.h>

int main() {
  auto zero = []() { return 42; };
  assert(zero() == 42);

  auto one = [](int x) { return x + 1; };
  assert(one(1) == 2);

  auto three = [](int x, int y, int z) { return x * 100 + y * 10 + z; };
  assert(three(1, 2, 3) == 123);

  return 0;
}
