#include <assert.h>

typedef int (*transform_t)(int);

int apply(int x, transform_t fn) { return fn(x); }

int main() {
  transform_t fresh = [](int x) { return -x; };
  assert(fresh(5) == -5);

  auto twice = [](int x) { return x * 2; };
  transform_t named = twice;
  assert(named(5) == 10);
  assert(apply(5, twice) == 10);

  named = fresh;
  assert(named(3) == -3);

  return 0;
}
