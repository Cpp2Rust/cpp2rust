#include <assert.h>
#include <stdarg.h>

struct pair {
  int a;
  int b;
};

int sum_mixed(int count, ...) {
  va_list ap;
  va_start(ap, count);
  int total = 0;
  for (int i = 0; i < count; i++) {
    int tag = va_arg(ap, int);
    if (tag == 0) {
      total += va_arg(ap, int);
    } else if (tag == 1) {
      total += (int)va_arg(ap, double);
    } else if (tag == 3) {
      struct pair p = va_arg(ap, struct pair);
      total += p.a * p.b;
    } else {
      long val = va_arg(ap, long);
      total += (int)val;
    }
  }
  va_end(ap);
  return total;
}

int main() {
  assert(sum_mixed(3, 0, 10, 1, 20.5, 2, 30L) == 60);
  assert(sum_mixed(1, 0, 42) == 42);
  assert(sum_mixed(2, 1, 3.7, 2, 100L) == 103);
  struct pair p = {7, 8};
  assert(sum_mixed(2, 3, p, 0, 5) == 61);
  return 0;
}
