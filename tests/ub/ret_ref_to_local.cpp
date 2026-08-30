// panic-ub: refcount

#include <cassert>

const int &foo() { return 5; }

int main() {
  int bar = foo();
  return 0;
}
