#include <cassert>

int foo() {
  static int static_i;
  static float static_f;
  static bool static_b;

  static int kX1 = 1;
  static const int kX2 = 2;
  kX1 += 1;
  return kX1 + kX2 + static_i;
}

int from_local_class() {
  static int x = 3;
  struct S {
    int get() const { return ++x; }
  };
  return S{}.get() + S{}.get();
}

int main() {
  assert(foo() + foo() + foo() == 15);
  assert(from_local_class() == 9);
  assert(from_local_class() == 13);
  return 0;
}
