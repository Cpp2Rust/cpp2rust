#include <assert.h>
#include <stdarg.h>

int conditional_log(int verbose, const char *fmt, ...) {
  if (verbose) {
    va_list ap;
    va_start(ap, fmt);
    int result = va_arg(ap, int);
    va_end(ap);
    return result;
  }
  return -1;
}

int main() {
  assert(conditional_log(1, "%d", 42) == 42);
  assert(conditional_log(0, "%d", 99) == -1);
  return 0;
}
