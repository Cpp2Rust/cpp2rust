#include <assert.h>
#include <stdarg.h>

int extract_nth(int n, va_list ap) {
  for (int i = 0; i < n; i++) {
    va_arg(ap, int);
  }
  return va_arg(ap, int);
}

int middle_layer(int n, va_list ap) {
  return extract_nth(n, ap);
}

int top_level(int n, ...) {
  va_list ap;
  va_start(ap, n);
  int result = middle_layer(n, ap);
  va_end(ap);
  return result;
}

int main() {
  assert(top_level(2, 100, 200, 300, 400) == 300);
  assert(top_level(0, 42, 99) == 42);
  assert(top_level(3, 1, 2, 3, 4) == 4);
  return 0;
}
