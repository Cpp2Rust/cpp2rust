// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// no-compile
#include <assert.h>
#include <stdarg.h>

typedef int (*transform_t)(int);

int apply(int x, transform_t fn) { return fn(x); }

template <auto F> int call_nttp(int x) { return F(x); }

int main() {
  transform_t fresh = [](int x) { return -x; };
  assert(fresh(5) == -5);

  auto twice = [](int x) { return x * 2; };
  transform_t named = twice;
  assert(named(5) == 10);
  assert(apply(5, twice) == 10);

  named = fresh;
  assert(named(3) == -3);

  auto p = +[](int x) { return x * 3; };
  assert(p(2) == 6);

  assert(call_nttp<+[](int x) { return x - 1; }>(5) == 4);

  auto sum = [](int n, ...) {
    va_list ap;
    va_start(ap, n);
    int s = 0;
    for (int i = 0; i < n; i++) {
      s += va_arg(ap, int);
    }
    va_end(ap);
    return s;
  };
  assert(sum(2, 5, 5) == 10);
  assert(sum(3, 1, 2, 3) == 6);

  return 0;
}
