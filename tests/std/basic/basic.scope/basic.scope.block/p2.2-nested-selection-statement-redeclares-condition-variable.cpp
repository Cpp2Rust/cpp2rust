// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/2.2: a declaration in the block scope of a
// substatement must not conflict with one in the parent scope, unless the
// substatement is itself a selection or iteration statement.
#include <cassert>

static int inner_then_runs;
static int inner_else_runs;
static int outer_else_runs;

int main() {
  if (int choice = 4)
    if (int choice = 0) {
      ++inner_then_runs;
    } else {
      ++inner_else_runs;
      assert(choice == 0);
    }
  else {
    ++outer_else_runs;
    assert(choice == 0);
  }
  assert(inner_then_runs == 0);
  assert(inner_else_runs == 1);
  assert(outer_else_runs == 0);
  return 0;
}
