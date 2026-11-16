// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.scope.lambda]/1: a lambda-expression introduces a lambda scope
// that starts immediately after its lambda-introducer and extends to the end
// of its compound-statement.
namespace p1_init_capture_hides_enclosing_variable {
void test() {
  int total = 4;
  auto bump = [total = total + 1]() mutable { return ++total; };
  assert(total == 4);
  assert(bump() == 6);
  assert(bump() == 7);
  assert(total == 4);
}
} // namespace p1_init_capture_hides_enclosing_variable

int main() {
  p1_init_capture_hides_enclosing_variable::test();
  return 0;
}
