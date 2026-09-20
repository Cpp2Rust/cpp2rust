#include <cassert>

struct Pair {
  int x, y;
};

int main() {
  int *x = new int(5);
  int out = *x;
  delete x;
  assert(out == 5);

  int *y = new int;
  *y = 9;
  assert(*y == 9);
  delete y;

  Pair *p = new Pair;
  p->x = 1;
  p->y = 2;
  assert(p->x + p->y == 3);
  delete p;
  return 0;
}
