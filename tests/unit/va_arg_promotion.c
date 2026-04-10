// C promotion rules: char/short promote to int, float promotes to double
#include <assert.h>
#include <stdarg.h>

int test_promotions(int count, ...) {
  va_list ap;
  va_start(ap, count);

  // char and short are promoted to int by the caller
  int a = va_arg(ap, int);   // was passed as char 'A' (65)
  int b = va_arg(ap, int);   // was passed as short 10

  // float is promoted to double by the caller
  double c = va_arg(ap, double);  // was passed as float 3.0

  va_end(ap);

  assert(a == 65);
  assert(b == 10);
  assert(c == 3.0);

  return a + b + (int)c;
}

int main() {
  char x = 'A';     // 65
  short y = 10;
  float z = 3.0f;
  // 65 + 10 + 3 = 78
  assert(test_promotions(3, x, y, z) == 78);
  return 0;
}
