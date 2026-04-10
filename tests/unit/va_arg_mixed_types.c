#include <stdarg.h>

double sum_mixed(int count, ...) {
  va_list ap;
  va_start(ap, count);
  double total = 0;
  for (int i = 0; i < count; i++) {
    total += va_arg(ap, double);
  }
  va_end(ap);
  return total;
}

int main() {
  return (int)sum_mixed(3, 1.5, 2.5, 3.0) - 7;
}
