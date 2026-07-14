#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct record {
  char *name;
};

int main(void) {
  char *d = strdup("hello");
  assert(d != NULL);
  assert(strcmp(d, "hello") == 0);
  free(d);
  const char *p = "world";
  char buf[] = {'a', 'b', 'c', '\0'};
  char *d2 = strdup(p);
  assert(d2 != NULL);
  assert(strcmp(d2, p) == 0);
  free(d2);
  char *d3 = strdup(buf);
  assert(d3 != NULL);
  assert(strcmp(d3, buf) == 0);
  free(d3);
  char *d4 = 0;
  d4 = strdup(p);
  assert(d4 != NULL);
  assert(strcmp(d4, p) == 0);
  free(d4);
  struct record rec = {0};
  struct record *r = &rec;
  r->name = strdup(p);
  assert(r->name != NULL);
  assert(strcmp(r->name, p) == 0);
  free(r->name);
  return 0;
}
