#include <assert.h>

int main() {
  auto zero = []() { return 42; };
  assert(zero() == 42);

  auto one = [](int x) { return x + 1; };
  assert(one(1) == 2);

  auto three = [](int x, int y, int z) { return x * 100 + y * 10 + z; };
  assert(three(1, 2, 3) == 123);

  int hits = 0;
  auto no_return = [&hits](int by) { hits += by; };
  no_return(3);
  no_return(4);
  assert(hits == 7);

  int a = 2;
  int b = 3;
  int product = [&]() { return a * b; }();
  assert(product == 6);

  return 0;
}
