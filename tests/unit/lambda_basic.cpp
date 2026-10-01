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

  return 0;
}
