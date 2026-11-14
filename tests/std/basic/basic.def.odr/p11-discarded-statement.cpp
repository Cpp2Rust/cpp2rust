// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/11: a definition is required only for functions
// odr-used outside of a discarded statement. undefined() is called only in a
// discarded statement, so it is never defined and never runs.
#include "../../support/lifetime-tracked.h"
#include <cassert>

static int else_runs;

LifetimeTracked undefined();

void g() {
  if constexpr (false) {
    undefined().use();
  } else {
    ++else_runs;
  }
}

int main() {
  g();
  assert(else_runs == 1);
  assert(LifetimeTracked::ctorCount == 0);
  assert(LifetimeTracked::useCount == 0);
  return 0;
}
