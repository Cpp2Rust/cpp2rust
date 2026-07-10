#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

struct Layout {
  uint8_t a;
  uint32_t b;
  uint16_t c;
};

struct Frame {
  uint16_t tag;
  char body[64];
};

int main(void) {
  assert(offsetof(struct Layout, a) == 0);
  assert(offsetof(struct Layout, b) == 4);
  assert(offsetof(struct Layout, c) == 8);

  struct Layout v = {0};
  v.b = 0xDEADBEEF;
  unsigned char *base = (unsigned char *)&v;
  uint32_t *bp = (uint32_t *)(base + offsetof(struct Layout, b));
  assert(*bp == 0xDEADBEEF);

  *(uint32_t *)(base + offsetof(struct Layout, b)) = 0x12345678;
  assert(v.b == 0x12345678);

  const char *text = "example-body";
  size_t len = strlen(text) + 1;
  size_t total = offsetof(struct Frame, body) + len;
  assert(total == 2 + len);

  return 0;
}
