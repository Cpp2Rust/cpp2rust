// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [conv.mem]/2: the null member pointer value is converted to the null
// member pointer value of the destination type.
#include <cassert>

struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
  void increment() { ++value; }
};

struct InheritsValue : Precedes, DeclaresValue {};

int main() {
  int DeclaresValue::*base_pm = nullptr;
  int InheritsValue::*derived_pm = base_pm;
  assert(derived_pm == nullptr);
  void (DeclaresValue::*base_pmf)() = nullptr;
  void (InheritsValue::*derived_pmf)() = base_pmf;
  assert(derived_pmf == nullptr);
  return 0;
}
