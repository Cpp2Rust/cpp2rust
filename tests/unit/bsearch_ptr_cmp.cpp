#include <cassert>
#include <cstdlib>

int int_cmp(const void *v1, const void *v2) {
  return *(const int *)v1 - *(const int *)v2;
}

int main() {
  int a1[] = {1, 2, 3};
  void *vptr1 = std::bsearch(&a1[0], a1, 1, sizeof(int), int_cmp);
  assert(vptr1 == &a1[0]);
  return 0;
}
