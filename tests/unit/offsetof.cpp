// no-compile: refcount
#include <assert.h>
#include <stddef.h>
#include <stdint.h>

struct Layout {
  uint8_t a;
  uint32_t b;
  uint16_t c;
};

struct Inner {
  uint16_t x;
  uint32_t y;
};

struct Outer {
  uint8_t pad;
  struct Inner inner;
};

int main(void) {
  assert(offsetof(struct Layout, a) == 0);
  assert(offsetof(struct Layout, b) == 4);
  assert(offsetof(struct Layout, c) == 8);

  assert(offsetof(struct Outer, inner.y) == 8);

  struct Layout v = {0};
  v.b = 0xDEADBEEF;
  unsigned char *base = (unsigned char *)&v;
  uint32_t *bp = (uint32_t *)(base + offsetof(struct Layout, b));
  assert(*bp == 0xDEADBEEF);

  return 0;
}
