#include <assert.h>
#include <string.h>

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

  unsigned char buf[sizeof(union basic)];
  memset(buf, 0, sizeof(buf));
  union basic *ru = reinterpret_cast<union basic *>(buf);
  int *pi = &ru->i;
  float *pf = &ru->f;

  ru->i = 7;
  assert(*pi == 7);

  *pi = 0x3F800000;
  assert(ru->i == 0x3F800000);
  assert(*pf == 1.0f);
  assert(buf[3] == 0x3F);

  return 0;
}
