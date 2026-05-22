#include <assert.h>
#include <limits.h>

int main() {
  long a = 0;
  assert(!__builtin_mul_overflow(3L, 7L, &a));
  assert(a == 21);

  long b = 0;
  assert(__builtin_mul_overflow(LONG_MAX, 2L, &b));

  long long c = 0;
  assert(!__builtin_mul_overflow(1000LL, 1000LL, &c));
  assert(c == 1000000);

  long long d = 0;
  assert(__builtin_mul_overflow(LLONG_MAX, 2LL, &d));

  return 0;
}
