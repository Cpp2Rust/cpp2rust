#include <assert.h>

int main() {
  int x1 = 0xFFFFFFFF;
  assert(x1 == -1);
  signed char x2 = 0xFF;
  assert(x2 == -1);

  unsigned int u1 = 5;
  unsigned int u2 = -u1;
  assert(u2 == 4294967291u);

  char c1 = 0xFF;
  assert((unsigned char)c1 == 0xFF);
  return 0;
}
