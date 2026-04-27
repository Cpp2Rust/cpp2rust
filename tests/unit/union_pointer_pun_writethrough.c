// translation-fail
#include <assert.h>

int main(void) {
  long x = -1;

  union {
    unsigned long *to_ulong;
    long *to_long;
  } lptr;

  lptr.to_long = &x;
  *lptr.to_ulong = 42UL;

  assert(x == 42);
  return 0;
}
