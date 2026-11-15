// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/5.1: only a reference that is usable in constant
// expressions is exempt. r refers to a local object, so it is odr-used and [=]
// captures it, which copies the object it refers to.
#include "../../support/lifetime-tracked.h"
#include <cassert>

int main() {
  LifetimeTracked t;
  LifetimeTracked &r = t;
  auto f = [=] { r.use(); };
  assert(LifetimeTracked::copyCount == 1);
  f();
  assert(LifetimeTracked::copyCount == 1);
  assert(LifetimeTracked::useCount == 1);
  assert(t.uses == 0);
  return 0;
}
