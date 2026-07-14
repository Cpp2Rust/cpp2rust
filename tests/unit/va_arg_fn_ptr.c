#include <assert.h>
#include <stdarg.h>

typedef int (*unary)(int);
typedef int (*binary)(int, int);

int square(int x) { return x * x; }
int negate(int x) { return -x; }
int add(int a, int b) { return a + b; }

int apply_unary(int x, ...) {
  va_list ap;
  va_start(ap, x);
  unary fn = va_arg(ap, unary);
  int result = fn(x);
  va_end(ap);
  return result;
}

int apply_binary(int a, int b, ...) {
  va_list ap;
  va_start(ap, b);
  binary fn = va_arg(ap, binary);
  int result = fn(a, b);
  va_end(ap);
  return result;
}

static int not_supported(void *ctx, unary fn, void *extra) {
  (void)ctx;
  (void)fn;
  (void)extra;
  return -3;
}

int main() {
  assert(apply_unary(5, square) == 25);
  assert(apply_unary(7, negate) == -7);
  assert(apply_binary(3, 4, add) == 7);
  int dummy = 0;
  assert(not_supported(&dummy, square, &dummy) == -3);
  return 0;
}
