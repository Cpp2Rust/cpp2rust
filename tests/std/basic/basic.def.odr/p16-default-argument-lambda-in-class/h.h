// N4950 [basic.def.odr]/16, Example 6: X is a class whose member function h
// has a lambda-expression in a default argument. Defined in several
// translation units, the lambda denotes the same closure type in each, so it
// has one static local variable.
#pragma once

#include "../../../support/lifetime-tracked.h"

struct X {
  void h(bool cond, void (*p)() = [] {
    static LifetimeTracked local;
    local.use();
  }) {
    if (cond)
      h(false);
    else
      p();
  }
};

void b_call();
