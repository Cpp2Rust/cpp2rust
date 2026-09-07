#include <assert.h>

struct S {
  S() : a(11), b(true) {}

  int a;
  bool b;
};

int main() {
  S s;
  assert(s.a == 11);
  assert(s.b == true);
  return 0;
}
