#include <assert.h>

static int retry(int n) {
  int count = 0;
  int acc = 0;
again:
  count += 1;
  acc += n;
  if (count < 3) {
    goto again;
  }
  return acc;
}

int main(void) {
  assert(retry(4) == 12);
  return 0;
}
