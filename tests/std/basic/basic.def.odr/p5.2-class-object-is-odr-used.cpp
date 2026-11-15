// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/5.2: the exemption covers only expressions of non-class
// type. Copying the constant k as a whole odr-uses it, so [=] captures it.
#include <cassert>

#include "../../support/lifetime-tracked.h"

int main() {
  constexpr HasConstexprCtor k{5};
  auto f = [=] {
    HasConstexprCtor c = k;
    return c.m;
  };
  assert(HasConstexprCtor::copyCount == 1);
  assert(f() == 5);
  assert(HasConstexprCtor::copyCount == 2);
  return 0;
}
