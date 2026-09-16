#include <assert.h>

int main() {
  int x = 10;

  auto outer = [&x](int y) {
    auto inner = [&x, y](int z) { return x + y + z; };
    return inner(1);
  };

  assert(outer(20) == 31);

  x = 100;
  assert(outer(20) == 121);

  return 0;
}
