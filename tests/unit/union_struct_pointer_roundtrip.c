#include <assert.h>
#include <stdint.h>

struct pair {
  int x;
  int y;
};

int main(void) {
  struct pair arr[3];
  arr[0].x = 10;
  arr[1].x = 20;
  arr[2].x = 30;

  union {
    struct pair *p;
    uintptr_t bits;
  } u;

  u.p = &arr[1];
  struct pair *q = u.p;
  assert(q->x == 20);
  assert(q == &arr[1]);

  u.bits += sizeof(struct pair);
  assert(u.p->x == 30);
  assert(u.p == &arr[2]);

  u.bits -= 2 * sizeof(struct pair);
  assert(u.p->x == 10);
  assert(u.p == &arr[0]);

  return 0;
}
