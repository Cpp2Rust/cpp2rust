#include <assert.h>
#include <stdarg.h>

int extract_first(char *buf, int size, const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  int n = va_arg(ap, int);
  buf[0] = (char)n;
  va_end(ap);
  return n;
}

int main() {
  char buf[64];
  assert(extract_first(buf, 1, "%d", 42) == 42);
  assert(buf[0] == 42);
  assert(extract_first(buf, 1, "%d", 65) == 65);
  assert(buf[0] == 'A');
  return 0;
}
