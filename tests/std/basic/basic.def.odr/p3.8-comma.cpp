// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/3.8: the potential results of a comma expression are
// those of its right operand. k is not odr-used, so [=] does not capture it;
// the left operand is still evaluated.
#include <cassert>

#include "../../support/lifetime-tracked.h"

static int left;

int main() {
  constexpr HasConstexprCtor k{5};
  auto f = [=] { return (++left, k).m; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(left == 0);
  assert(f() == 5);
  assert(left == 1);
  assert(HasConstexprCtor::copyCount == 0);
  return 0;
}
