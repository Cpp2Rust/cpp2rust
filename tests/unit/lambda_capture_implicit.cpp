#include <assert.h>

int main() {
  int a = 1;
  int b = 2;
  int c = 3;

  auto by_value = [=](int x) { return a + b + c + x; };
  assert(by_value(10) == 16);
  a = 100;
  assert(by_value(10) == 16);

  auto by_ref = [&](int x) { return a + b + c + x; };
  assert(by_ref(10) == 115);
  b = 200;
  assert(by_ref(10) == 313);

  auto mixed = [=, &c](int x) {
    c += x;
    return a + b + c;
  };
  assert(mixed(1) == 100 + 200 + 4);
  assert(c == 4);

  return 0;
}
