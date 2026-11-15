// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/8: a function is odr-used if it is named by a
// potentially-evaluated expression. make() is named only in unevaluated
// operands, so it is never defined; noisy() is defined, but an unevaluated call
// does not run it.
#include <cassert>

#include "../../support/lifetime-tracked.h"

static int noisy_calls;

LifetimeTracked make();

int noisy() {
  ++noisy_calls;
  return 1;
}

int main() {
  assert(sizeof(make()) == sizeof(LifetimeTracked));
  assert(sizeof(noisy()) == sizeof(int));
  assert(!noexcept(make()));
  assert(!noexcept(noisy()));
  decltype(noisy()) n = 3;
  assert(n == 3);
  assert(noisy_calls == 0);
  assert(LifetimeTracked::ctorCount == 0);
  assert(noisy() == 1);
  assert(noisy_calls == 1);
  return 0;
}
