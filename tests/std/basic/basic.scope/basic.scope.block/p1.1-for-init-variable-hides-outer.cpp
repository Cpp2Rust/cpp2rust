// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/1.1: each selection or iteration statement
// introduces a block scope that includes that statement.
#include <cassert>

int main() {
  int index = 42;
  int iterations = 0;
  int sum = 0;
  for (int index = 0; index < 3; ++index) {
    ++iterations;
    sum += index;
  }
  assert(iterations == 3);
  assert(sum == 3);
  assert(index == 42);
  return 0;
}
