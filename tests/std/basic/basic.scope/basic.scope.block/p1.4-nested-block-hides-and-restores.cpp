// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/1.4: each compound statement that is not the
// compound-statement of a handler introduces a block scope that includes that
// statement.
#include <cassert>

#include "../../../support/lifetime-tracked.h"

int main() {
  LifetimeTracked tracked;
  {
    LifetimeTracked tracked;
    assert(LifetimeTracked::ctorCount == 2);
    tracked.use();
    assert(tracked.uses == 1);
    assert(LifetimeTracked::dtorCount == 0);
  }
  assert(LifetimeTracked::dtorCount == 1);
  assert(tracked.uses == 0);

  tracked.use();
  assert(tracked.uses == 1);
  assert(LifetimeTracked::useCount == 2);
  return 0;
}
