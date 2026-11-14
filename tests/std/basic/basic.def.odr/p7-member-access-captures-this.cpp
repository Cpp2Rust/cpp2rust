// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/7: *this is odr-used if this appears as a
// potentially-evaluated expression, including through the implicit
// transformation of a member name. [=] captures this, not the member x, and the
// lambda sees later changes to the object.
#include "../../support/lifetime-tracked.h"
#include <cassert>

struct HasMemberUsedInLambda {
  LifetimeTracked x;
  int n = 1;
  auto makeLambda() {
    return [=] {
      x.use();
      return n;
    };
  }
};

int main() {
  HasMemberUsedInLambda c;
  auto f = c.makeLambda();
  assert(LifetimeTracked::copyCount == 0);
  assert(f() == 1);
  assert(c.x.uses == 1);
  c.n = 10;
  assert(f() == 10);
  assert(c.x.uses == 2);
  assert(LifetimeTracked::copyCount == 0);
  return 0;
}
