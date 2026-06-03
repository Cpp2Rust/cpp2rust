#include <assert.h>

static int loopctl(void) {
  int sum = 0;
  for (int i = 0; i < 5; i++) {
    if (i == 1) {
      continue;
    }
    if (i == 4) {
      break;
    }
    goto add;
  add:
    sum += 1;
  }
  return sum;
}

int main(void) {
  assert(loopctl() == 3);
  return 0;
}
