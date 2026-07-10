#include <assert.h>
#include <stdint.h>
#include <string.h>

struct point {
  int32_t x;
  int32_t y;
};

int main(void) {
  struct point src = {3, 7};

  unsigned char buf[sizeof(struct point)];
  memcpy(buf, &src, sizeof(buf));

  struct point dst;
  memcpy(&dst, buf, sizeof(dst));

  assert(dst.x == 3);
  assert(dst.y == 7);

  return 0;
}
