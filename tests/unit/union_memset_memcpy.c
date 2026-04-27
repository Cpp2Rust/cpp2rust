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
  uint32_t addr;
  char zero[8];
};

struct Storage {
  union {
    struct header_a a;
    struct header_b b;
    char raw[256];
  } buffer;
};

int main(void) {
  struct Storage s;

  memset(&s, 0, sizeof(s));
  assert(s.buffer.a.tag == 0);
  assert(s.buffer.b.port == 0);
  assert(s.buffer.raw[0] == 0);
  assert(s.buffer.raw[255] == 0);

  unsigned char wire[16] = {0};
  wire[0] = 2;
  wire[2] = 0x50;
  wire[3] = 0x00;
  wire[4] = 0x7F;
  wire[5] = 0x00;
  wire[6] = 0x00;
  wire[7] = 0x01;
  size_t len = 16;
  assert(len <= sizeof(s.buffer.raw));
  memcpy(&s.buffer.raw, wire, len);

  assert(s.buffer.b.tag == 2);
  assert(((unsigned char *)&s.buffer.b.port)[0] == 0x50);

  memset(&s, 0, sizeof(s));
  assert(s.buffer.b.tag == 0);

  return 0;
}
