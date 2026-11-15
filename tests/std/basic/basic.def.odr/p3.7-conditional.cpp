// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/3.7: the potential results of a glvalue conditional
// expression are those of its second and third operands. Neither k1 nor k2 is
// odr-used, so [=] captures neither.
#include <cassert>

#include "../../support/lifetime-tracked.h"

int main() {
  constexpr HasConstexprCtor k1{7};
  constexpr HasConstexprCtor k2{9};
  auto f = [=](bool b) { return (b ? k1 : k2).m; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(f(true) == 7);
  assert(f(false) == 9);
  assert(HasConstexprCtor::copyCount == 0);
  return 0;
}
