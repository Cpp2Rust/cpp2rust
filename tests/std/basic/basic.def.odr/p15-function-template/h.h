// N4950 [basic.def.odr]/15: a template defined in several translation units
// behaves as a single entity, so the same specialization instantiated in two
// translation units has one set of static local variables, constructed once and
// destroyed once. Different specializations have their own.
#pragma once

#include "../../../support/lifetime-tracked.h"

template <class T> int count() {
  static LifetimeTracked t;
  static int n = 0;
  return ++n;
}

int b_count_int();
int b_count_long();
