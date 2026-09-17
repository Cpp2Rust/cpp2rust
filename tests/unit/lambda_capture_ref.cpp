#include <assert.h>
#include <stddef.h>
#include <stdint.h>

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

  uint16_t arr[4] = {3, 1, 2, 0};
  auto swap = [&arr](size_t i, size_t j) {
    uint16_t t = arr[j];
    arr[j] = arr[i];
    arr[i] = t;
  };
  swap(0, 3);
  assert(arr[0] == 0);
  assert(arr[3] == 3);

  return 0;
}
