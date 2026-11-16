// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.param]/1.1: the scope of a function's parameters extends
// to the end of the function-definition.
#include <cassert>

int rate = 3;

int scale(int rate) {
  rate *= 2;
  return rate;
}

int read_rate() { return rate; }

int main() {
  assert(scale(5) == 10);
  assert(rate == 3);
  assert(read_rate() == 3);
  return 0;
}
