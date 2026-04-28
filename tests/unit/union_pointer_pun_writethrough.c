// panic: refcount
// XFAIL: unsafe
#include <assert.h>

int main(void) {
  long x = -1;

  union {
    unsigned long *as_unsigned;
    long *as_signed;
  } pp;

  pp.as_signed = &x;
  *pp.as_unsigned = 42UL;

  assert(x == 42);
  return 0;
}
