#include <assert.h>

static int value = 5;

int param_shadow(int value) { return value + 1; }

int local_shadow(void) {
  int value = 99;
  return value;
}

int read_global(void) { return value; }

int main() {
  assert(param_shadow(10) == 11);
  assert(local_shadow() == 99);
  assert(read_global() == 5);
  return 0;
}
