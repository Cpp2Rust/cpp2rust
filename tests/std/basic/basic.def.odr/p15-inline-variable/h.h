// N4950 [basic.def.odr]/15: an inline variable defined in several translation
// units behaves as a single entity: one object with one address, constructed
// once and destroyed once.
#pragma once

#include "../../../support/asserts-dtor-count-at-exit.h"
#include "../../../support/lifetime-tracked.h"

inline AssertsDtorCountAtExit check{1};
inline LifetimeTracked shared;

void b_use();
LifetimeTracked *b_address();
