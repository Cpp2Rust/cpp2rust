// N4950 [basic.def.odr]/14.5.1: an inline function defined in several
// translation units may name a const object with internal linkage, although
// that is a different object in each translation unit, provided the object has
// the same value everywhere and is only read by value. factor is such an
// object: each file has its own, and scaled is still one function.
#pragma once

const int factor = 3;

inline int scaled(int v) { return v * factor; }

const int *b_factor_address();
int (*b_scaled_address())(int);
int b_scaled(int v);
