// panic: refcount
#include <assert.h>

struct pair {
  int a[4];
  int b[4];
};

int main(void) {
  struct pair s;

  assert(s.a + 4 == s.b);
  return 0;
}
