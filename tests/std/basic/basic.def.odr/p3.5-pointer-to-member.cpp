// XFAIL: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/3.5: the potential results of E1.*E2 are those of E1.
// Reading k.*pm does not odr-use k, so [=] does not capture it.
#include <cassert>

#include "../../support/lifetime-tracked.h"

int main() {
  constexpr HasConstexprCtor k{5};
  constexpr int HasConstexprCtor::*pm = &HasConstexprCtor::m;
  auto f = [=] { return k.*pm; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(f() == 5);
  assert(HasConstexprCtor::copyCount == 0);
  return 0;
}
