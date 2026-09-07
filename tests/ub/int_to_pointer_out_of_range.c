// panic-ub: refcount
// nondet-result: unsafe

#include <stdint.h>

int main(void) {
  int arr[4] = {1, 2, 3, 4};

  union {
    int *p;
    uintptr_t bits;
  } u;

  u.p = arr;
  u.bits += 100 * sizeof(int);
  int *p = u.p;

  return *p == 0 ? 0 : 1;
}
