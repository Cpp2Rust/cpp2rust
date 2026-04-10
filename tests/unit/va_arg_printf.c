#include <assert.h>
#include <stdarg.h>

int logf_impl(const char *fmt, va_list ap) {
  (void)fmt;
  return va_arg(ap, int) + va_arg(ap, int);
}

int logf(const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  int result = logf_impl(fmt, ap);
  va_end(ap);
  return result;
}

int main() {
  assert(logf("hello %d %d", 10, 32) == 42);
  assert(logf("x %d %d", 1, 2) == 3);
  return 0;
}
