#include <cassert>

struct Pair {
  int x, y;
};

struct Triple {
  int a;
  int b;
  Pair p;
};

int main() {
  Pair *p = new Pair{1, 2};
  int out = p->x + p->y;
  delete p;
  assert(out == 3);

  Triple t{1};
  assert(t.a == 1);
  assert(t.b == 0);
  assert(t.p.x == 0 && t.p.y == 0);

  Triple *q = new Triple{2, 3};
  assert(q->a == 2);
  assert(q->b == 3);
  assert(q->p.x == 0 && q->p.y == 0);
  delete q;
  return 0;
}
