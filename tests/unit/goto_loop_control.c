#include <assert.h>

static int loopctl(int n) {
  int total = 0;
  for (int i = 0; i < n; i++) {
    if (i == 2) {
      continue;
    }
    if (i == 5) {
      break;
    }
    if (i % 2 == 0) {
      goto even;
    }
    total += 1;
  even:
    total += 10;
  }
  return total;
}

int main(void) {
  assert(loopctl(10) == 42);
  return 0;
}
