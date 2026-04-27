// translation-fail
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

struct header {
  uint16_t tag;
  uint16_t port;
  uint32_t addr;
  char pad[8];
};

struct Storage {
  union {
    struct header h;
    char raw[128];
  } buffer;
};

static void external_writer(void *out, size_t cap) {
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
  struct Storage s;
  memset(&s, 0, sizeof(s));

  external_writer((void *)&s.buffer, sizeof(s.buffer));

  assert(s.buffer.h.tag == 2);
  assert(((unsigned char *)&s.buffer.h.port)[0] == 0x00);
  assert(((unsigned char *)&s.buffer.h.port)[1] == 0x50);

  assert(s.buffer.raw[0] == 2);
  assert((unsigned char)s.buffer.raw[3] == 0x50);

  return 0;
}
