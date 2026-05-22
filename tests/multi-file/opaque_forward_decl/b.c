#include "header.h"

void touch(struct container *c) {
  /* read the opaque pointer field without dereferencing the opaque type */
  (void)c->p;
}
