// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.lambda]/1: a lambda-expression introduces a lambda scope
// that starts immediately after its lambda-introducer and extends to the end
// of its compound-statement.
#include <cassert>

int main() {
  int total = 4;
  auto bump = [total = total + 1]() mutable { return ++total; };
  assert(total == 4);
  assert(bump() == 6);
  assert(bump() == 7);
  assert(total == 4);
  return 0;
}
