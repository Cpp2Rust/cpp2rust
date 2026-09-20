#include <assert.h>

struct S {
  S() : a(11), b(true) {}

  int a;
  bool b;
};

struct Declared {
  int v;
  Declared();
};

int main() {
  Declared *d = nullptr;
  assert(d == nullptr);
  S s;
  assert(s.a == 11);
  assert(s.b == true);
  return 0;
}
