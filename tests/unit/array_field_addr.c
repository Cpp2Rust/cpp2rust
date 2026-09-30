#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct S {
  int before;
  unsigned char mask[4];
  int after;
};

int main(void) {
  struct S *s = malloc(sizeof(struct S));
  assert(s != NULL);
  s->before = 1;
  memset(s->mask, 5, sizeof(s->mask));
  s->after = 2;
  *(unsigned char *)&s->mask = 7;
  unsigned char out[4];
  memcpy(out, &s->mask, sizeof(s->mask));
  assert(out[0] == 7 && out[3] == 5);
  assert(s->before == 1 && s->after == 2);
  free(s);
  return 0;
}
