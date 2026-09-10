#include <cassert>

int foo(int a, int b = 10) { return a + b; }

bool baz(int *a, int *b = nullptr) { return a == b; }

int main() {
  assert(foo(1) == 11);
  assert(foo(1, 2) == 3);

  int a = 0;
  assert(baz(&a) == false);
  assert(baz(&a, &a) == true);

  return 0;
}
