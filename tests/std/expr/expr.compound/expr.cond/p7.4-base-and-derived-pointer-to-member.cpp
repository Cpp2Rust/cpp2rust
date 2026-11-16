// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.cond]/7.4: if one or both of the second and third operands have
// pointer-to-member type, pointer-to-member conversions bring them to their
// composite pointer type.
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
  int InheritsValue::*derived_pm = &InheritsValue::own;
  InheritsValue obj;
  obj.preceding = 1;
  obj.value = 2;
  obj.own = 3;
  bool choose_base = true;
  assert(obj.*(choose_base ? base_pm : derived_pm) == 2);
  choose_base = false;
  assert(obj.*(choose_base ? base_pm : derived_pm) == 3);
  return 0;
}
