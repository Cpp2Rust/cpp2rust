// panic-ub: refcount
// nondet-result: unsafe

#include <stdint.h>

int main(void) {
  union {
    int *p;
    uintptr_t bits;
  } u;

  u.bits = 0xdeadbeef;
  int *p = u.p;

  return *p == 0 ? 0 : 1;
}
