#include <assert.h>
#include <stddef.h>

static void write_ulong(unsigned long *p) { *p = 42; }

int main(void) {
  size_t x = 0;
  write_ulong(&x);
  assert(x == 42);
  return 0;
}
