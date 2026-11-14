// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/5.3: a variable named by a discarded-value expression
// to which the lvalue-to-rvalue conversion is not applied is not odr-used, so
// [=] does not capture t.
#include "../../support/lifetime-tracked.h"
#include <cassert>

int main() {
  LifetimeTracked t;
  auto f = [=] { (void)t; };
  assert(LifetimeTracked::copyCount == 0);
  f();
  assert(LifetimeTracked::copyCount == 0);
  assert(LifetimeTracked::ctorCount == 1);
  return 0;
}
