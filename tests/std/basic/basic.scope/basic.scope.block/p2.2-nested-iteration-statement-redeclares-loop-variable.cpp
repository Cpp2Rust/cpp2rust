// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/2.2: a declaration in the block scope of a
// substatement must not conflict with one in the parent scope, unless the
// substatement is itself a selection or iteration statement.
#include <cassert>

int main() {
  int outer_iterations = 0;
  int inner_iterations = 0;
  int sum = 0;
  for (int index = 0; index < 2; ++index, ++outer_iterations)
    for (int index = 10; index < 13; ++index) {
      ++inner_iterations;
      sum += index;
    }
  assert(outer_iterations == 2);
  assert(inner_iterations == 6);
  assert(sum == 66);
  return 0;
}
