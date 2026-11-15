// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/11: a definition is required only for functions
// odr-used outside of a discarded statement. In a template the discarded
// branch is not instantiated; undefined() is called only there, so it is never
// defined and never runs.
#include "../../support/lifetime-tracked.h"
#include <cassert>

static int else_runs;

LifetimeTracked undefined();

template <class T> void g(T) {
  if constexpr (sizeof(T) > 1000) {
    undefined().use();
  } else {
    ++else_runs;
  }
}

int main() {
  g(1);
  g('c');
  assert(else_runs == 2);
  assert(LifetimeTracked::ctorCount == 0);
  assert(LifetimeTracked::useCount == 0);
  return 0;
}
