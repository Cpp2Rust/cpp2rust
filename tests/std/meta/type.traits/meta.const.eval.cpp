// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <type_traits>

// N4950 [meta.const.eval]/1: Effects: Equivalent to:
//   if consteval {
//     return true;
//   } else {
//     return false;
//   }
namespace p1_is_constant_evaluated {
int pick() { return std::is_constant_evaluated() ? 1 : 2; }

void test() {
  constexpr bool compile_time = std::is_constant_evaluated();
  assert(compile_time);
  bool run_time = std::is_constant_evaluated();
  assert(!run_time);
  assert(pick() == 2);
}
} // namespace p1_is_constant_evaluated

int main() {
  p1_is_constant_evaluated::test();
  return 0;
}
