#include <assert.h>
#include <string.h>

int main(void) {
  char buf[6] = {1, 2, 3, 4, 5, 6};

  memmove(buf + 2, buf, 4);

  assert(buf[0] == 1);
  assert(buf[1] == 2);
  assert(buf[2] == 1);
  assert(buf[3] == 2);
  assert(buf[4] == 3);
  assert(buf[5] == 4);

  return 0;
}
