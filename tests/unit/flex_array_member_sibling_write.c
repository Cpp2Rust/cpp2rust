// panic: refcount
#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct S {
  int n;
  char name[1];
};

struct E {
  int id;
  int w;
};

struct T {
  int n;
  int cap;
  struct E a[1];
};

int main(void) {
  struct S *s = calloc(1, sizeof(struct S) + 8);
  assert(s != NULL);
  memcpy(s->name, "abcdefg", 8);
  s->n = 5;
  assert(s->n == 5);
  assert(strcmp(s->name, "abcdefg") == 0);
  free(s);

  struct T *t = malloc(sizeof(struct T) + sizeof(struct E));
  assert(t != NULL);
  t->n = 2;
  t->cap = 2;
  t->a[0].id = 10;
  t->a[1].w = 20;
  t->n = 3;
  assert(t->a[0].id == 10 && t->a[1].w == 20);
  struct E *tail = (struct E *)&t[1];
  assert(tail == &t->a[1]);
  tail[0].id = 30;
  t->cap = 4;
  assert(t->a[1].id == 30 && t->a[1].w == 20);
  assert(t->n == 3 && t->cap == 4);
  free(t);
  return 0;
}
