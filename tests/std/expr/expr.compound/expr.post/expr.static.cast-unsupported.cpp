// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.static.cast]/13: "pointer to member of D of type T" can be
// converted to "pointer to member of B of type T", B a base class of D; the
// result points to the original member.
namespace p13_derived_to_base_data_member {
struct LacksValue {
  int own;
};

struct AddsValue : LacksValue {
  int value;
};

void test() {
  int LacksValue::*pm = static_cast<int LacksValue::*>(&AddsValue::value);
  AddsValue obj;
  obj.own = 1;
  obj.value = 2;
  LacksValue &base = obj;
  assert(base.*pm == 2);
  base.*pm = 3;
  assert(obj.value == 3);
  assert(obj.own == 1);
}
} // namespace p13_derived_to_base_data_member

// N4950 [expr.static.cast]/13: "pointer to member of D of type T" can be
// converted to "pointer to member of B of type T", B a base class of D; the
// result points to the original member.
namespace p13_derived_to_base_member_function {
struct LacksIncrement {
  int own;
};

struct AddsIncrement : LacksIncrement {
  int value;
  void increment() { ++value; }
};

void test() {
  void (LacksIncrement::*pmf)() =
      static_cast<void (LacksIncrement::*)()>(&AddsIncrement::increment);
  AddsIncrement obj;
  obj.own = 1;
  obj.value = 2;
  LacksIncrement &base = obj;
  (base.*pmf)();
  assert(obj.value == 3);
  assert(obj.own == 1);
}
} // namespace p13_derived_to_base_member_function

// N4950 [expr.static.cast]/13: the null member pointer value is converted to
// the null member pointer value of the destination type.
namespace p13_null_converts_to_null {
struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
};

struct InheritsValue : Precedes, DeclaresValue {};

void test() {
  int InheritsValue::*derived_pm = nullptr;
  int DeclaresValue::*base_pm = static_cast<int DeclaresValue::*>(derived_pm);
  assert(base_pm == nullptr);
}
} // namespace p13_null_converts_to_null

// N4950 [expr.static.cast]/13: if class B contains the original member, the
// resulting pointer to member points to the original member.
namespace p13_round_trip_yields_original {
struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
};

struct InheritsValue : Precedes, DeclaresValue {};

void test() {
  int InheritsValue::*derived_pm = &DeclaresValue::value;
  int DeclaresValue::*base_pm = static_cast<int DeclaresValue::*>(derived_pm);
  assert(base_pm == &DeclaresValue::value);
  DeclaresValue obj{2};
  assert(obj.*base_pm == 2);
}
} // namespace p13_round_trip_yields_original

int main() {
  p13_derived_to_base_data_member::test();
  p13_derived_to_base_member_function::test();
  p13_null_converts_to_null::test();
  p13_round_trip_yields_original::test();
  return 0;
}
