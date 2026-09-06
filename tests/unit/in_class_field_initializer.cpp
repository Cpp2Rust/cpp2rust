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
};

int main() {
  S s{};
  assert(s.a == 1);
  assert(s.b == 2);
  assert(s.c.x == 3);
  assert(s.c.y == 4);
  return 0;
};
