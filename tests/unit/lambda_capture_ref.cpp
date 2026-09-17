#include <assert.h>

struct S {
  int x;
  int y;
};

int main() {
  int base = 10;
  auto add_base = [&base](int x) { return x + base; };
  assert(add_base(5) == 15);
  base = 100;
  assert(add_base(5) == 105);

  S s = {1, 2};
  auto sum = [&s]() { return s.x + s.y; };
  assert(sum() == 3);
  s.x = 50;
  assert(sum() == 52);

  int counter = 0;
  auto bump = [&counter]() { counter++; };
  bump();
  bump();
  assert(counter == 2);

  return 0;
}
