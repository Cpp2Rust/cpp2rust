#include <cassert>
#include <string>

struct Inner {
  int x = 3;
  int y = 4;
};

struct S {
  int a = 1;
  char b = 2;
  Inner c = {};
  Inner d;
};

template <typename T> struct Boxed {
  T v = T();
  int tag = 7;
  Boxed(T x, int t) : v(x), tag(t) {}
};

int main() {
  S s;
  assert(s.a == 1);
  assert(s.b == 2);
  assert(s.c.x == 3);
  assert(s.c.y == 4);
  assert(s.d.x == 3);
  assert(s.d.y == 4);
  Boxed<int> boxed(5, 9);
  assert(boxed.v == 5);
  assert(boxed.tag == 9);
  return 0;
};
