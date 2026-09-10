#include <cassert>

template <typename T> T zero() { return T(); }

template <typename T> void destroy(T *p) { p->~T(); }

int main() {
  int i = int();
  double d = double{};
  int *p = zero<int *>();
  assert(i == 0);
  assert(d == 0.0);
  assert(p == nullptr);
  assert(zero<long>() == 0);

  int x = 5;
  destroy(&x);
  using I = int;
  x.~I();
  assert(x == 5);
  return 0;
}
