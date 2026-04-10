#include <assert.h>
#include <stdarg.h>

int sum_ints(int first, ...) {
  va_list ap;
  int total = first;

  va_start(ap, first);
  int val;
  while ((val = va_arg(ap, int)) != 0) {
    total += val;
  }
  va_end(ap);

  return total;
}

int main() {
  assert(sum_ints(1, 2, 3, 4, 0) == 10);
  assert(sum_ints(100, 0) == 100);
  assert(sum_ints(5, 5, 5, 5, 5, 0) == 25);
  return 0;
}
