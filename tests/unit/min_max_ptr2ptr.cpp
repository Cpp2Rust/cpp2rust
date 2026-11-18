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

  int values[2] = {2, 1};
  int *low = &values[0];
  int *high = &values[1];
  assert(std::min(low, high) == low);
  assert(std::min(high, low) == low);
  assert(std::max(low, high) == high);
  assert(std::max(high, low) == high);
  return 0;
}
