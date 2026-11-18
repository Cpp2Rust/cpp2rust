#include <cassert>

#include "../../../support/asserts-dtor-count-at-exit.h"
#include "h.h"

AssertsDtorCountAtExit check{1};

int main() {
  assert(LifetimeTracked::ctorCount == 0);
  instance().use();
  b_use();
  assert(LifetimeTracked::ctorCount == 1);
  assert(LifetimeTracked::useCount == 2);
  assert(instance().uses == 2);
  assert(&instance() == b_address());
  assert(LifetimeTracked::ctorCount == 1);
  assert(LifetimeTracked::copyCount == 0);
  return 0;
}
