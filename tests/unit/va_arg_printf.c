#include <assert.h>
#include <stdarg.h>
#include <string.h>

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

int lenf(const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  const char *s = va_arg(ap, const char *);
  int result = (int)strlen(s);
  va_end(ap);
  return result;
}

int main() {
  const char *dummy = "dummy";
  assert(logf("hello %d %d", 10, strlen(dummy)) == 15);
  assert(logf("x %d %d", 1, 2) == 3);
  assert(lenf("%s", dummy) == 5);
  assert(lenf("%s", dummy[0] ? dummy : "") == 5);
  return 0;
}
