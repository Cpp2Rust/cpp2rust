#include <cassert>

int main() {
  const int x = 0;
  assert(__builtin_constant_p(x));
  assert(__builtin_constant_p(3) + 2 == 3);
  return 0;
}
