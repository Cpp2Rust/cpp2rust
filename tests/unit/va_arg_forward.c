#include <stdarg.h>

int inner(int count, va_list ap) {
  int total = 0;
  for (int i = 0; i < count; i++) {
    total += va_arg(ap, int);
  }
  return total;
}

int outer(int count, ...) {
  va_list ap;
  va_start(ap, count);
  int result = inner(count, ap);
  va_end(ap);
  return result;
}

int main() {
  return outer(3, 10, 20, 30) - 60;
}
