#include <assert.h>
#include <stddef.h>
#include <stdint.h>

int main(void) {
  union {
    int *p;
    uintptr_t bits;
  } u;

  u.bits = 0;
  assert(u.p == NULL);

  int x = 5;
  u.p = &x;
  assert(u.bits != 0);

  u.bits = 0;
  assert(u.p == NULL);

  u.p = NULL;
  assert(u.bits == 0);

  return 0;
}
