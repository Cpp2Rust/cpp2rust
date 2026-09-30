#include <assert.h>
#include <stdlib.h>

struct In {
  int x;
  int y;
};

struct S {
  struct In in;
  int total;
  int n;
  int arr[4];
};

struct Node {
  int x;
  struct Node *self;
};

int main(void) {
  struct S *p = calloc(1, sizeof(struct S));
  assert(p != NULL);
  struct S *q = p;
  p->in.x = 1;
  p->in.y = 2;
  p->total = q->in.x + q->in.y;
  assert(q->total == 3);
  struct In *ip = &p->in;
  ip->x = p->total + 1;
  assert(q->in.x == 4 && q->in.y == 2);
  p->arr[p->n] = p->total;
  p->n += 1;
  p->arr[p->n] = q->in.x;
  assert(q->arr[0] == 3 && q->arr[1] == 4 && q->n == 1);
  free(p);

  struct Node s;
  s.x = 1;
  s.self = &s;
  s.self->x = s.x + 1;
  assert(s.x == 2);
  return 0;
}
