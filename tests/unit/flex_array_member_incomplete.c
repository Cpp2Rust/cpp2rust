#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct F {
  int n;
  char tail[];
};

int main(void) {
  assert(sizeof(struct F) == sizeof(int));
  struct F *f = malloc(sizeof(struct F) + 4);
  assert(f != NULL);
  f->n = 4;
  memcpy(f->tail, "xyz", 4);
  assert(f->n == 4);
  assert(strcmp(f->tail, "xyz") == 0);
  assert(f->tail[1] == 'y');
  free(f);
  struct F *g = malloc(sizeof(struct F) + 8);
  assert(g != NULL);
  g->n = 8;
  memcpy(g->tail, "abcdefg", 8);
  assert(g->tail[6] == 'g');
  assert(g->tail[7] == '\0');
  free(g);
  return 0;
}
