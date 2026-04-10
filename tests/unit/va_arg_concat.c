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
  return sum_ints(1, 2, 3, 4, 0) - 10;
}
