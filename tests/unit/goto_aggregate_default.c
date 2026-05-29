#include <assert.h>
#include <string.h>

static int agg(int n) {
  char buf[40];
  int total = 0;
  if (n < 0) {
    goto out;
  }
  memset(buf, 1, sizeof(buf));
  for (int i = 0; i < 40; i++) {
    total += buf[i];
  }
out:
  return total;
}

int main(void) {
  assert(agg(-1) == 0);
  assert(agg(2) == 40);
  return 0;
}
