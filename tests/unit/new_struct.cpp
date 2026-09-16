#include <cassert>

struct Pair {
  int x, y;
};

int main() {
  Pair *p = new Pair{1, 2};
  int out = p->x + p->y;
  delete p;
  assert(out == 3);
  return 0;
}
