// Pattern from wget's concat_strings: va_start twice to iterate args in two passes
#include <assert.h>
#include <stdarg.h>

int sum_then_product(int first, ...) {
  va_list ap;
  int sum = first;
  int product = first;

  // First pass: compute sum
  va_start(ap, first);
  int val;
  while ((val = va_arg(ap, int)) != 0) {
    sum += val;
  }
  va_end(ap);

  // Second pass: compute product
  va_start(ap, first);
  while ((val = va_arg(ap, int)) != 0) {
    product *= val;
  }
  va_end(ap);

  return sum + product;
}

int main() {
  // sum = 2+3+4 = 9, product = 2*3*4 = 24, total = 33
  assert(sum_then_product(2, 3, 4, 0) == 33);
  return 0;
}
