// translation-fail
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

struct record {
  uint16_t code;
  uint16_t lo;
  uint32_t hi;
  char pad[8];
};

struct Container {
  union {
    struct record h;
    char raw[128];
  } view;
};

static void fill(void *out, size_t cap) {
  unsigned char src[16] = {0};
  src[0] = 2;
  src[1] = 0;
  src[2] = 0x00;
  src[3] = 0x50;
  src[4] = 0x7F;
  src[5] = 0x00;
  src[6] = 0x00;
  src[7] = 0x01;
  size_t n = sizeof(src) < cap ? sizeof(src) : cap;
  memcpy(out, src, n);
}

int main(void) {
  struct Container c;
  memset(&c, 0, sizeof(c));

  fill((void *)&c.view, sizeof(c.view));

  assert(c.view.h.code == 2);
  assert(((unsigned char *)&c.view.h.lo)[0] == 0x00);
  assert(((unsigned char *)&c.view.h.lo)[1] == 0x50);

  assert(c.view.raw[0] == 2);
  assert((unsigned char)c.view.raw[3] == 0x50);

  return 0;
}
