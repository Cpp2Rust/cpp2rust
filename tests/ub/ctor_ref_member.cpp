// panic-ub: refcount

#include <cassert>

struct S {
  const int &r;
  S(const int &x) : r(x) {}
};

int main() {
  S s(5);
  assert(s.r == 5);
  return 0;
}
