// translation-fail: refcount
// The refcount converter asserts on a constant array that a lambda uses
// without capturing it.
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/3.2: the potential results of a subscripting operation
// with an array operand are those of the operand. Reading arr[1].m does not
// odr-use arr, so [=] does not capture it.
#include <cassert>

#include "../../support/lifetime-tracked.h"

int main() {
  constexpr HasConstexprCtor arr[2] = {3, 4};
  auto f = [=] { return arr[1].m; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(f() == 4);
  assert(HasConstexprCtor::copyCount == 0);
  return 0;
}
