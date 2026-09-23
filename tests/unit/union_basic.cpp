#include <assert.h>

union basic {
  int i;
  float f;
};

union empty {};

int main(void) {
  union basic u;
  union empty e;
  (void)e;

  u.i = 42;
  assert(u.i == 42);

  u.f = 3.14f;
  assert(u.f == 3.14f);

  return 0;
}
