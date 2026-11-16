// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/13: the null member pointer value is converted to
// the null member pointer value of the destination type.
#include <cassert>

struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
};

struct InheritsValue : Precedes, DeclaresValue {};

int main() {
  int InheritsValue::*derived_pm = nullptr;
  int DeclaresValue::*base_pm = static_cast<int DeclaresValue::*>(derived_pm);
  assert(base_pm == nullptr);
  return 0;
}
