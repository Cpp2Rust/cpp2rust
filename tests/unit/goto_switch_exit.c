#include <assert.h>

static int classify(int n) {
  int ret = 0;
  switch (n) {
  case 1:
    ret = 10;
    goto out;
  case 2:
    ret = 20;
    break;
  default:
    ret = 30;
    break;
  }
  ret += 1;
out:
  return ret;
}

int main(void) {
  assert(classify(1) == 10);
  assert(classify(2) == 21);
  assert(classify(9) == 31);
  return 0;
}
