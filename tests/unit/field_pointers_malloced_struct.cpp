#include <cassert>
#include <cstdlib>

struct S {
  int a;
  int b;
  int c;
};

static int bump(S *s) {
  s->b += 10;
  return s->b;
}

int main() {
  S *s = static_cast<S *>(calloc(1, sizeof(S)));
  assert(s != nullptr);
  s->b = 1;
  s->a = bump(s);
  assert(s->a == 11);
  assert(s->b == 11);
  s->a = 1;
  s->b = 2;
  s->c = 0;
  if (s->a < s->b && s->c++ == 0) {
    s->a = 5;
  }
  assert(s->a == 5 && s->c == 1);
  if (s->a < s->b && s->c++ == 0) {
    s->a = 6;
  }
  assert(s->a == 5 && s->c == 1);
  int x = s->a + (s->b = 3);
  assert(x == 8 && s->b == 3);
  int y = 0;
  s->c = (y = 99);
  assert(s->c == 99 && y == 99);
  s->a += bump(s);
  assert(s->a == 18 && s->b == 13 && s->c == 99);
  free(s);
  return 0;
}
