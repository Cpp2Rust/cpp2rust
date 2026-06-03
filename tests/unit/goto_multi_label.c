#include <assert.h>

static int classify(int n) {
  int ret = 0;
  if (n < 0) {
    goto error;
  }
  if (n == 0) {
    goto out;
  }
  ret = n;
  goto out;
error:
  ret = -1;
out:
  return ret;
}

int main(void) {
  assert(classify(5) == 5);
  assert(classify(0) == 0);
  assert(classify(-2) == -1);
  return 0;
}
