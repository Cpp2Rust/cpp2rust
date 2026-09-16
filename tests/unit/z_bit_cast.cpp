#include <assert.h>
void decay_cast(unsigned int *a1) {}
void bit_cast(const void *p) {}
int main() {
  unsigned int a1[] = {1, 2, 3};
  decay_cast(a1);
  decay_cast(&a1[0]);
  bit_cast(a1);
  bit_cast(&a1[0]);
  bit_cast(&a1);
  void *ptr = a1;
  assert(ptr == a1);
  assert(((unsigned int *)ptr)[0] == a1[0]);
  return 0;
}
