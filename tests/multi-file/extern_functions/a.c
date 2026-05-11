#include <assert.h>

extern int helper(int x);

int main(void) {
  assert(helper(42) == 43);
  return 0;
}
