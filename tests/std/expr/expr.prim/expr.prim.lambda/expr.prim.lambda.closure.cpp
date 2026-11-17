// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.prim.lambda.closure]/3: the closure type of a generic lambda has
// a function call operator template whose return type is described by the
// trailing-return-type.
namespace p3_call_operator_template_named_in_decltype {
void test() {
  auto twice = [](auto x) -> int { return x * 2; };
  decltype(twice(2.5)) n = 4;
  assert(n == 4);
  assert(twice(3) == 6);
}
} // namespace p3_call_operator_template_named_in_decltype

int main() {
  p3_call_operator_template_named_in_decltype::test();
  return 0;
}
