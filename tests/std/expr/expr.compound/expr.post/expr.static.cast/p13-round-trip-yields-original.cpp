// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/13: if class B contains the original member, the
// resulting pointer to member points to the original member.
#include <cassert>

struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
};

struct InheritsValue : Precedes, DeclaresValue {};

int main() {
  int InheritsValue::*derived_pm = &DeclaresValue::value;
  int DeclaresValue::*base_pm = static_cast<int DeclaresValue::*>(derived_pm);
  assert(base_pm == &DeclaresValue::value);
  DeclaresValue obj{2};
  assert(obj.*base_pm == 2);
  return 0;
}
