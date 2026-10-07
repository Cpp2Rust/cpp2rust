// no-compile: refcount
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
  return 0;
}
