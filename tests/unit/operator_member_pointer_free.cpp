#include <cassert>

struct Inner {
  int x;
};

struct S {
  int data[3];
  Inner inner;
};

Inner &operator*(S &s) { return s.inner; }
int *operator&(S &s) { return &s.data[0]; }

int main() {
  S s{{1, 2, 3}, {9}};
  assert((*s).x == 9);
  (*s).x = 10;
  assert(s.inner.x == 10);
  int *p = &s;
  assert(*p == 1);
  *p = 5;
  assert(s.data[0] == 5);
  return 0;
}
