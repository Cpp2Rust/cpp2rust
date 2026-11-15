// no-compile: refcount, unsafe
// Inside the lambda the converter refers to the original structured binding,
// not to the captured copy.
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/6: a structured binding is odr-used if it appears as a
// potentially-evaluated expression, so [=] captures a by copy.
#include "../../support/lifetime-tracked.h"
#include <cassert>

struct HasTwoMembers {
  LifetimeTracked a;
  LifetimeTracked b;
};

int main() {
  HasTwoMembers p{LifetimeTracked(), LifetimeTracked()};
  assert(LifetimeTracked::ctorCount == 2);
  auto &[a, b] = p;
  assert(LifetimeTracked::copyCount == 0);
  auto f = [=] { a.use(); };
  assert(LifetimeTracked::copyCount == 1);
  f();
  assert(LifetimeTracked::copyCount == 1);
  assert(LifetimeTracked::useCount == 1);
  assert(p.a.uses == 0);
  return 0;
}
