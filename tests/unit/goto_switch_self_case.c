#include <assert.h>

static int sm(int n) {
  int steps = 0;
  switch (n) {
  target:
  case 0:
    steps += 1;
    break;
  case 1:
    steps += 10;
    goto target;
  default:
    steps = -1;
    break;
  }
  return steps;
}

int main(void) {
  assert(sm(0) == 1);
  assert(sm(1) == 11);
  assert(sm(7) == -1);
  return 0;
}
