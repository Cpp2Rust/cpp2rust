#include <cassert>

#include "h.h"

int main() {
  shared.use();
  b_use();
  assert(LifetimeTracked::ctorCount == 1);
  assert(LifetimeTracked::useCount == 2);
  assert(shared.uses == 2);
  assert(&shared == b_address());
  assert(LifetimeTracked::copyCount == 0);
  return 0;
}
