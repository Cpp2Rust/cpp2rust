#include <assert.h>

static int scan(int n) {
  int total = 0;
  for (int i = 0; i < n; i++) {
    int j = 0;
    while (j < 10) {
      if (j == 5) {
        goto next;
      }
      total += 1;
      j++;
    }
    total += 100;
  next:
    total += 1000;
  }
  return total;
}

int main(void) {
  assert(scan(2) == 2010);
  return 0;
}
