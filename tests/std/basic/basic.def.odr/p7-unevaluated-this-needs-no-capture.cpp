// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/7: *this is odr-used only if this appears as a
// potentially-evaluated expression. In sizeof the member name is unevaluated,
// so the lambda needs no capture.
#include "../../support/lifetime-tracked.h"
#include <cassert>

struct HasMemberUsedInLambda {
  LifetimeTracked x;
  auto sizer() {
    return [] { return sizeof(x); };
  }
};

int main() {
  HasMemberUsedInLambda c;
  auto f = c.sizer();
  assert(f() == sizeof(LifetimeTracked));
  assert(LifetimeTracked::copyCount == 0);
  assert(LifetimeTracked::useCount == 0);
  return 0;
}
