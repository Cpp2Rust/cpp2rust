// N4950 [basic.def.odr]/15: an inline function defined in several translation
// units behaves as a single entity with a single definition: one address and
// one static local variable, constructed once and destroyed once.
#pragma once

#include "../../../support/lifetime-tracked.h"

inline LifetimeTracked &instance() {
  static LifetimeTracked t;
  return t;
}

void b_use();
LifetimeTracked *b_address();
