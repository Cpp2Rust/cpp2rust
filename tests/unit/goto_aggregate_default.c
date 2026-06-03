#include <assert.h>
#include <stdio.h>

struct Point {
  int x;
  int y;
};

static int agg(int n) {
  char buf40[40];
  unsigned char buf256[256];
  int arr64[64];
  long longs[33];
  struct Point p;
  int *ptr;
  int (*fp)(int);
  FILE *file;
  int total = 0;
  if (n < 0) {
    goto out;
  }
  total = 1;
out:
  return total;
}

int main(void) {
  assert(agg(-1) == 0);
  assert(agg(1) == 1);
  return 0;
}
