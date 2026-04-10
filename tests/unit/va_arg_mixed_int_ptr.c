// va_arg with mixed int and pointer types in the same call
#include <assert.h>
#include <stdarg.h>

int mixed_args(int count, ...) {
  va_list ap;
  va_start(ap, count);

  int total = 0;
  for (int i = 0; i < count; i++) {
    int tag = va_arg(ap, int);
    if (tag == 0) {
      total += va_arg(ap, int);
    } else {
      int *ptr = va_arg(ap, int *);
      total += *ptr;
    }
  }

  va_end(ap);
  return total;
}

int main() {
  int x = 100;
  // tag=0 val=10, tag=1 ptr=&x(100), tag=0 val=20
  assert(mixed_args(3, 0, 10, 1, &x, 0, 20) == 130);

  int y = 50;
  assert(mixed_args(1, 1, &y) == 50);
  assert(mixed_args(2, 0, 5, 0, 3) == 8);
  return 0;
}
