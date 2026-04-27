// translation-fail
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

struct header_a {
  uint16_t tag;
  char data[14];
};

struct header_b {
  uint16_t tag;
  uint16_t port;
  uint32_t flags;
  uint8_t body[16];
  uint32_t extra;
};

struct Storage {
  unsigned int len;
  union {
    struct header_a a;
    struct header_b b;
    char raw[64];
  } u;
};

int main(void) {
  struct Storage s;
  memset(&s, 0, sizeof(s));

  s.u.a.tag = 10;
  s.len = sizeof(struct header_b);

  ((struct header_b *)(void *)&s.u.a)->extra = 0xDEADBEEF;

  assert(s.u.b.extra == 0xDEADBEEF);
  assert(s.u.b.tag == 10);

  s.u.b.port = 0x1F90;
  assert(((unsigned char *)&s.u.raw)[2] == 0x90);
  assert(((unsigned char *)&s.u.raw)[3] == 0x1F);

  return 0;
}
