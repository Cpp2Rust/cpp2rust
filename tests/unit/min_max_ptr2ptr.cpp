#include <algorithm>
#include <cassert>

int main() {
  int a = 10, b = 20;
  int *pa = &a, *pb = &b;
  int **ppa = &pa;
  int **ppb = &pb;
  int r1 = std::max(**ppa, **ppb);
  int r2 = std::min(**ppa, **ppb);
  assert(r1 + r2 == 30);
  return 0;
}
