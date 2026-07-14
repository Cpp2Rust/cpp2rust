#include <assert.h>

static int early(int n) {
  int ret = 0;
  if (n < 0) {
    ret = -1;
    goto out;
  }
  ret = 100;
  const int intentionally_const_var = 22;
out:
  return ret + intentionally_const_var - intentionally_const_var;
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

struct wrapper {
  int *item;
};

static int via_pointer(struct wrapper *w, int fail) {
  int ret = 0;
  int *item = w->item;
  if (fail) {
    ret = -1;
    goto out;
  }
  ret = *item;
out:
  return ret;
}

static int via_arrays(int fail) {
  int ret = 0;
  unsigned char remain[4] = {0};
  char name[5] = "wxyz";
  if (fail) {
    ret = -1;
    goto out;
  }
  remain[1] = 9;
  ret = remain[0] + remain[1] + (name[0] == 'w') + (name[4] == '\0');
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
  int value = 42;
  struct wrapper w = {&value};
  assert(via_pointer(&w, 0) == 42);
  assert(via_pointer(&w, 1) == -1);
  assert(via_arrays(0) == 11);
  assert(via_arrays(1) == -1);
  return 0;
}
