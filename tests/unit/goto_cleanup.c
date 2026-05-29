#include <assert.h>

static int early(int n) {
  int ret = 0;
  if (n < 0) {
    ret = -1;
    goto out;
  }
  ret = 100;
out:
  return ret;
}

static int from_loop(int n) {
  int ret = 0;
  for (int i = 0; i < n; i++) {
    if (i == 3) {
      ret = 7;
      goto out;
    }
    ret += i;
  }
  ret = 999;
out:
  return ret;
}

static int from_switch(int n) {
  int ret = 0;
  switch (n) {
  case 1:
    ret = 10;
    goto out;
  default:
    ret = 20;
    break;
  }
  ret = 999;
out:
  return ret;
}

int main(void) {
  assert(early(-1) == -1);
  assert(early(5) == 100);
  assert(from_loop(2) == 999);
  assert(from_loop(10) == 7);
  assert(from_switch(1) == 10);
  assert(from_switch(2) == 999);
  return 0;
}
