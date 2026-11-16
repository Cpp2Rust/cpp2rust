// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/1.2: each substatement of a selection or iteration
// statement introduces a block scope that includes that substatement.
#include <cassert>

#include "../../../support/lifetime-tracked.h"

int main() {
  LifetimeTracked scoped;
  assert(LifetimeTracked::ctorCount == 1);

  for (int index = 0; index < 3; ++index)
    LifetimeTracked scoped;
  assert(LifetimeTracked::ctorCount == 4);
  assert(LifetimeTracked::dtorCount == 3);

  if (LifetimeTracked::ctorCount == 4)
    LifetimeTracked scoped;
  assert(LifetimeTracked::ctorCount == 5);
  assert(LifetimeTracked::dtorCount == 4);

  if (LifetimeTracked::ctorCount == 0)
    ;
  else
    LifetimeTracked scoped;
  assert(LifetimeTracked::ctorCount == 6);
  assert(LifetimeTracked::dtorCount == 5);

  scoped.use();
  assert(scoped.uses == 1);
  return 0;
}
