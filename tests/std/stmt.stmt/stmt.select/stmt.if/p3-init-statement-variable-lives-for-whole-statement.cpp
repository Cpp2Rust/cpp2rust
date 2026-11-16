// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [stmt.if]/3: if (init-statement condition) statement else statement
// is equivalent to a block holding the init-statement and the if, except that
// the init-statement is in the same scope as the condition.
#include <cassert>

#include "../../../support/lifetime-tracked.h"

static int then_runs;
static int else_runs;

int main() {
  int choice = 9;
  if (LifetimeTracked guard; int choice = 0) {
    ++then_runs;
  } else {
    ++else_runs;
    guard.use();
    assert(guard.uses == 1);
    assert(choice == 0);
    assert(LifetimeTracked::dtorCount == 0);
  }
  assert(then_runs == 0);
  assert(else_runs == 1);
  assert(LifetimeTracked::dtorCount == 1);

  if (LifetimeTracked guard; int choice = 4) {
    ++then_runs;
    guard.use();
    assert(guard.uses == 1);
    assert(choice == 4);
    assert(LifetimeTracked::dtorCount == 1);
  }
  assert(then_runs == 1);
  assert(LifetimeTracked::ctorCount == 2);
  assert(LifetimeTracked::dtorCount == 2);
  assert(choice == 9);
  return 0;
}
