#include <assert.h>
#include <stdarg.h>
#include <stddef.h>

int first_nonnull(int count, ...) {
  va_list ap;
  va_start(ap, count);
  int result = -1;
  for (int i = 0; i < count; i++) {
    int *p = va_arg(ap, int *);
    if (p != NULL) {
      result = *p;
      break;
    }
  }
  va_end(ap);
  return result;
}

int main() {
  int x = 42;
  assert(first_nonnull(2, NULL, &x) == 42);
  assert(first_nonnull(3, NULL, NULL, &x) == 42);
  assert(first_nonnull(1, NULL) == -1);
  return 0;
}
