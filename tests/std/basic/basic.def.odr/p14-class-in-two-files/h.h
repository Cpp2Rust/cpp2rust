// N4950 [basic.def.odr]/14 and /15: a class may be defined in several
// translation units. A member function defined in the class and an inline
// static data member are then single entities: one static local variable and
// one object, each constructed once and destroyed once.
#pragma once

#include "../../../support/asserts-dtor-count-at-exit.h"
#include "../../../support/lifetime-tracked.h"

inline AssertsDtorCountAtExit check{2};

struct HasInlineMembers {
  static inline LifetimeTracked shared;

  static LifetimeTracked &instance() {
    static LifetimeTracked local;
    return local;
  }
};

void b_use();
LifetimeTracked *b_shared_address();
LifetimeTracked *b_instance_address();
