#include <assert.h>

static int skip(int n) {
  int x = 0;
  if (n > 0) {
    goto mid;
  }
  x += 10;
mid:
  x += 1;
  return x;
}

int main(void) {
  assert(skip(1) == 1);
  assert(skip(-1) == 11);
  return 0;
}
