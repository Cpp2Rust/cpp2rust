// panic: refcount
#include <assert.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

struct S {
  int keep;
  int a;
  long b;
  char c[5];
  int last;
};

int main(void) {
  struct S *p = malloc(sizeof(struct S));
  assert(p != NULL);
  memset(p, 0xff, sizeof(*p));
  p->keep = 7;
  memset(&p->a, 0, sizeof(struct S) - offsetof(struct S, a));
  assert(p->keep == 7);
  assert(p->a == 0 && p->b == 0 && p->c[4] == 0 && p->last == 0);
  p->a = 1;
  p->b = 2;
  p->c[0] = 'x';
  p->last = 3;
  memset(&p->b, 0, offsetof(struct S, last) - offsetof(struct S, b));
  assert(p->keep == 7 && p->a == 1);
  assert(p->b == 0 && p->c[0] == 0);
  assert(p->last == 3);
  free(p);
  return 0;
}
