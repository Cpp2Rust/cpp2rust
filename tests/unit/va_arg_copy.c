// va_copy: save position, consume from copy, original still usable
#include <assert.h>
#include <stdarg.h>

int sum_with_copy(int count, ...) {
  va_list ap, aq;
  va_start(ap, count);

  // Copy before consuming
  va_copy(aq, ap);

  // Consume from original
  int sum1 = 0;
  for (int i = 0; i < count; i++) {
    sum1 += va_arg(ap, int);
  }
  va_end(ap);

  // Consume from copy (should start from same position)
  int sum2 = 0;
  for (int i = 0; i < count; i++) {
    sum2 += va_arg(aq, int);
  }
  va_end(aq);

  assert(sum1 == sum2);
  return sum1 + sum2;
}

int main() {
  // sum1 = 10+20+30 = 60, sum2 = 10+20+30 = 60, total = 120
  assert(sum_with_copy(3, 10, 20, 30) == 120);
  return 0;
}
