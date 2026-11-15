// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/5.2: a constant variable of non-class type read by
// value is not odr-used, so the lambda uses n with an empty capture list.
#include <cassert>

int main() {
  const int n = 5;
  auto f = [] { return n; };
  assert(f() == 5);
  return 0;
}
