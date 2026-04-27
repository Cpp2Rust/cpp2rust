// translation-fail
#include <assert.h>
#include <string.h>

struct Inner {
  int a;
  int b;
};

static int sum_inner(struct Inner *i) { return i->a + i->b; }

struct Outer {
  union {
    struct Inner inner;
    char raw[16];
  } u;
};

int main(void) {
  struct Inner standalone;
  standalone.a = 3;
  standalone.b = 4;
  assert(sum_inner(&standalone) == 7);

  struct Outer outer;
  memset(&outer, 0, sizeof(outer));
  outer.u.inner.a = 3;
  outer.u.inner.b = 4;
  assert(sum_inner(&outer.u.inner) == 7);

  assert((unsigned char)outer.u.raw[0] == 3);
  assert((unsigned char)outer.u.raw[4] == 4);

  return 0;
}
