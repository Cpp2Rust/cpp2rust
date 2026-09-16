#include <cassert>

int foo() { return 0; }

int main() {
  int x = foo() + 1;
  int y = foo();
  assert(x + y == 1);
  return 0;
}
