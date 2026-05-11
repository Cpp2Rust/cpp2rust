#include <assert.h>
#include <stdbool.h>

int main() {
  unsigned int flag = 7;
  bool b1 = (bool)flag;
  bool b2 = (bool)0u;
  assert(b1);
  assert(!b2);
  return 0;
}
