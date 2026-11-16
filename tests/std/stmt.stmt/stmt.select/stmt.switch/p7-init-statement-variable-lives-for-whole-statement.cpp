// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [stmt.switch]/7: switch (init-statement condition) statement is
// equivalent to a block holding the init-statement and the switch, except
// that the init-statement is in the same scope as the condition.
#include <cassert>

#include "../../../support/lifetime-tracked.h"

static int matching_case_runs;
static int other_case_runs;

int main() {
  int selector = 9;
  switch (LifetimeTracked guard; int selector = 2) {
  case 2:
    ++matching_case_runs;
    guard.use();
    assert(guard.uses == 1);
    assert(selector == 2);
    assert(LifetimeTracked::dtorCount == 0);
    break;
  default:
    ++other_case_runs;
    break;
  }
  assert(matching_case_runs == 1);
  assert(other_case_runs == 0);
  assert(LifetimeTracked::ctorCount == 1);
  assert(LifetimeTracked::dtorCount == 1);
  assert(selector == 9);
  return 0;
}
