// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [conv.mem]/2: indirection through the converted pointer with a D
// object is the same as indirecting through the pointer to member of B with
// the B subobject of D.
#include <cassert>

struct Precedes {
  int preceding;
};

struct DeclaresIncrement {
  int value;
  void increment() { ++value; }
};

struct InheritsIncrement : Precedes, DeclaresIncrement {
  int own;
};

int main() {
  void (DeclaresIncrement::*base_pmf)() = &DeclaresIncrement::increment;
  void (InheritsIncrement::*derived_pmf)() = base_pmf;
  InheritsIncrement obj;
  obj.preceding = 1;
  obj.value = 2;
  obj.own = 3;
  (obj.*derived_pmf)();
  assert(obj.value == 3);
  assert(obj.preceding == 1);
  assert(obj.own == 3);
  return 0;
}
