// no-compile
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

struct shape_a {
  uint16_t code;
  char pad[14];
};

struct shape_b {
  uint16_t code;
  uint16_t lo;
  uint32_t mid;
  uint8_t fill[16];
  uint32_t tail;
};

struct Container {
  unsigned int len;
  union {
    struct shape_a a;
    struct shape_b b;
    char raw[64];
  } u;
};

int main(void) {
  struct Container c;
  memset(&c, 0, sizeof(c));

  c.u.a.code = 10;
  c.len = sizeof(struct shape_b);

  ((struct shape_b *)(void *)&c.u.a)->tail = 0xDEADBEEF;

  assert(c.u.b.tail == 0xDEADBEEF);
  assert(c.u.b.code == 10);

  c.u.b.lo = 0x1F90;
  assert(((unsigned char *)&c.u.raw)[2] == 0x90);
  assert(((unsigned char *)&c.u.raw)[3] == 0x1F);

  return 0;
}
