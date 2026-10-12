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

struct Holder {
  S items[2];
};

struct FromStatic {
  static constexpr int kInit = 3;
  int v = kInit;
};

int main() {
  Declared *d = nullptr;
  assert(d == nullptr);
  S s;
  assert(s.a == 11);
  assert(s.b == true);

  Holder *h = new Holder[1];
  assert(h[0].items[1].a == 11);
  delete[] h;
  FromStatic *fs = new FromStatic[2];
  assert(fs[1].v == 3);
  delete[] fs;
  return 0;
}
