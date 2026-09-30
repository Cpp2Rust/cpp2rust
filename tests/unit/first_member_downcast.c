// panic: refcount
#include <assert.h>
#include <stdlib.h>

typedef struct base {
  int kind;
} base;

typedef struct derived {
  base head;
  size_t value;
} derived;

int main(void) {
  derived *d = malloc(sizeof(*d));
  assert(d != NULL);
  d->head.kind = 3;
  d->value = 7;
  base *b = &d->head;
  derived *back = (derived *)b;
  assert(back == d);
  assert(back->value == 7);
  assert(back->head.kind == 3);
  back->value = 8;
  assert(d->value == 8);
  b->kind = 4;
  assert(d->head.kind == 4);
  free(back);
  return 0;
}
