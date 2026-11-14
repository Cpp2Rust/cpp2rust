// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/3.3: the potential results of E1.E2 naming a non-static
// data member are those of E1. Reading k.m does not odr-use k, so [=] does not
// capture it.
#include "../../support/lifetime-tracked.h"
#include <cassert>

int main() {
  constexpr HasConstexprCtor k{5};
  auto f = [=] { return k.m; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(f() == 5);
  assert(HasConstexprCtor::copyCount == 0);
  return 0;
}
