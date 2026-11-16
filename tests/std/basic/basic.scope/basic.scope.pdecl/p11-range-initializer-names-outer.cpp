// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.pdecl]/11: the locus of a for-range-declaration of a
// range-based for statement is immediately after the for-range-initializer.
#include <cassert>

int main() {
  int values[3] = {2, 5, 9};
  int steps = 0;
  int sum = 0;
  for (int values : values) {
    values *= 2;
    ++steps;
    sum += values;
  }
  assert(steps == 3);
  assert(sum == 32);
  assert(values[0] == 2);
  assert(values[1] == 5);
  assert(values[2] == 9);
  return 0;
}
