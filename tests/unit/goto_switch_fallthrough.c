#include <assert.h>

static int sm(int n) {
  int ret = 0;
  switch (n) {
  case 0:
    ret += 1;
    /* fallthrough */
  case 1:
    ret += 10;
    goto out;
  default:
    ret += 100;
    break;
  }
  ret += 1000;
out:
  return ret;
}

int main(void) {
  assert(sm(0) == 11);
  assert(sm(1) == 10);
  assert(sm(9) == 1100);
  return 0;
}
