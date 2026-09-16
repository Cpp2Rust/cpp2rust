#include <cassert>

int main() {
  int a[] = {1, 2, 3, 4, 5};
  const int *p0 = &a[0];
  const int *p1 = &a[4];
  assert(p1 - p0 == 4);
  return 0;
}
