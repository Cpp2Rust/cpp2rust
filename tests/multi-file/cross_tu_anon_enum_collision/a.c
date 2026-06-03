#include <assert.h>
// anon enum below shares the same declaration line as b.c:enum { BETA = 9 }
enum { ALPHA = 7 };

int b_value(void);

int a_value(void) {
  int x = 0;
  x |= ALPHA;
  return x;
}

int main(void) {
  assert(a_value() == 7);
  assert(b_value() == 9);
  return 0;
}
