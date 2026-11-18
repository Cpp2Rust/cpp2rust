// N4950 [basic.def.odr]/16, Example 6: f is an inline function containing a
// lambda-expression. Defined in several translation units, it behaves as one
// definition, so the lambda inside it is one closure type with one static
// local variable.
#pragma once

#include "../../../support/lifetime-tracked.h"

inline void f(bool cond, void (*p)()) {
  if (cond)
    f(false, [] {
      static LifetimeTracked local;
      local.use();
    });
  else
    p();
}

void b_call();
