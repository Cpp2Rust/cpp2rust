#include <cassert>

struct Pod {
  int v;
};

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

  Pod pod{7};
  destroy(&pod);
  pod.~Pod();
  assert(pod.v == 7);
  return 0;
}
