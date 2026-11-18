// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.prim.lambda.capture]/12: an entity is captured by reference if
// it is implicitly or explicitly captured but not captured by copy.
namespace p12_parameter_of_generic_lambda_captured_by_reference {
void test() {
  auto increment_twice = [](auto n) {
    auto increment = [&n] { ++n; };
    increment();
    increment();
    return n;
  };
  assert(increment_twice(3) == 5);
}
} // namespace p12_parameter_of_generic_lambda_captured_by_reference

int main() {
  p12_parameter_of_generic_lambda_captured_by_reference::test();
  return 0;
}
