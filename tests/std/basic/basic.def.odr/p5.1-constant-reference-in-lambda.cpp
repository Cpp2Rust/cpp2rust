// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/5.1: a reference that is usable in constant expressions
// is not odr-used. [=] therefore does not capture r, and the lambda uses the
// object r refers to.
#include "../../support/lifetime-tracked.h"
#include <cassert>

LifetimeTracked global;

int main() {
  constexpr LifetimeTracked &r = global;
  auto f = [=] { r.use(); };
  assert(LifetimeTracked::copyCount == 0);
  f();
  assert(LifetimeTracked::copyCount == 0);
  assert(global.uses == 1);
  assert(LifetimeTracked::ctorCount == 1);
  return 0;
}
