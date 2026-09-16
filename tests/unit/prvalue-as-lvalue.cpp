#include <cassert>

const int &foo(const int &a) { return a; }

int main() {
  int a = 1, *pa = &a;
  const int &b = foo(*pa);
  assert(b == 1);
  return 0;
}
