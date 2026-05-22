#include <assert.h>
#include <stddef.h>

#include "header.h"

int main(void) {
  struct container c = {NULL, 42};
  touch(&c);
  assert(c.x == 42);
  assert(c.p == NULL);
  return 0;
}
