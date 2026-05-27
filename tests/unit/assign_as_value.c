#include <assert.h>

int main(void) {
  char buf[2];
  char *p = buf, *q;
  q = p += 1;
  assert(q == buf + 1);

  char out;
  switch (out = 'x') {
  case 'x':
    assert(1);
    break;
  default:
    assert(0);
    break;
  }
  assert(out == 'x');
  return 0;
}
