// N4950 [basic.def.odr]/15: a static data member of a class template may be
// defined in several translation units and behaves as a single entity: one
// object per specialization.
#pragma once

#include "../../../support/lifetime-tracked.h"

template <class T> struct HasStaticMember {
  static LifetimeTracked member;
};

template <class T> LifetimeTracked HasStaticMember<T>::member;

void b_use_int();
void b_use_long();
LifetimeTracked *b_int_address();
