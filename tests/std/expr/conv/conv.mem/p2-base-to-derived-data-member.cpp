// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [conv.mem]/2: "pointer to member of B of type T" converts to "pointer
// to member of D of type T"; the result refers to the member in D's instance
// of B.
#include <cassert>

struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
};

struct InheritsValue : Precedes, DeclaresValue {
  int own;
};

int main() {
  int DeclaresValue::*base_pm = &DeclaresValue::value;
  int InheritsValue::*derived_pm = base_pm;
  InheritsValue obj;
  obj.preceding = 1;
  obj.value = 2;
  obj.own = 3;
  assert(obj.*derived_pm == 2);
  obj.*derived_pm = 4;
  assert(obj.value == 4);
  assert(obj.preceding == 1);
  assert(obj.own == 3);
  return 0;
}
