#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct In {
  short a;
  short b;
};

struct S {
  int x;
  struct In in;
  int z;
};

int main(void) {
  struct S src = {1, {2, 3}, 4};
  struct S *p = malloc(sizeof(struct S));
  assert(p != NULL);
  memcpy(&p->x, &src, sizeof(*p));
  assert(p->x == 1 && p->in.a == 2 && p->in.b == 3 && p->z == 4);
  struct In n = {5, 6};
  memcpy(&p->in, &n, sizeof(n));
  assert(p->x == 1 && p->in.a == 5 && p->in.b == 6 && p->z == 4);
  unsigned char *bz = (unsigned char *)&p->z;
  for (int i = 0; i < 4; i++) {
    bz[i] = 1;
  }
  assert(p->z == 0x01010101 && p->in.b == 6);
  free(p);
  return 0;
}
