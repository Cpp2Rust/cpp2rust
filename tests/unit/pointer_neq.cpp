#include <cassert>

int main() {
  int x = 5;
  const int *p1 = &x;
  const int *p2 = &x;
  assert(!(p1 != p2));
  return 0;
}
