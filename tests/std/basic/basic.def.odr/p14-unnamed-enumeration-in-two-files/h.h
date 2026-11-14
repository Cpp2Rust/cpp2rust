// N4950 [basic.def.odr]/14: an unnamed enumeration may be defined in several
// translation units if every definition satisfies the requirements; the
// definitions then denote one type, so `current` is one object of that type.
#pragma once

enum { A = 1, B };

extern decltype(A) current;

void b_set_b();
