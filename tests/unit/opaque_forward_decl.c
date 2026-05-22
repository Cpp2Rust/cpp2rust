#include <stddef.h>

/* Forward declaration only; no body for `opaque` anywhere in this TU. */
struct opaque;

struct container {
  struct opaque *p;
  int x;
};

int main(void) {
  struct container c = {NULL, 42};
  (void)c.p;
  return c.x - 42;
}
