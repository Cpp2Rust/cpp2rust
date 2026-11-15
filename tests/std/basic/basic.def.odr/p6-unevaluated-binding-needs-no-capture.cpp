// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/6: a structured binding is odr-used only if it appears
// as a potentially-evaluated expression. In sizeof it is unevaluated, so [=]
// does not capture a.
#include <cassert>

#include "../../support/lifetime-tracked.h"

struct HasTwoMembers {
  LifetimeTracked a;
  LifetimeTracked b;
};

int main() {
  HasTwoMembers p{LifetimeTracked(), LifetimeTracked()};
  auto &[a, b] = p;
  auto f = [=] { return sizeof(a); };
  assert(LifetimeTracked::copyCount == 0);
  assert(f() == sizeof(LifetimeTracked));
  assert(LifetimeTracked::copyCount == 0);
  assert(LifetimeTracked::useCount == 0);
  return 0;
}
