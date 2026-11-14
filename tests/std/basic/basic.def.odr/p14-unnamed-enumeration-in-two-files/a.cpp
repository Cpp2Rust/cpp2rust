#include "h.h"
#include <cassert>

int main() {
  current = A;
  assert(current == 1);
  b_set_b();
  assert(current == B);
  assert(current == 2);
  return 0;
}
