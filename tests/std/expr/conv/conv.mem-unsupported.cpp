// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [conv.mem]/1: the null member pointer value is distinguishable from
// any pointer to member not created from a null pointer constant.
namespace p1_null_distinct_from_member_function {
struct HasVirtualAndNonVirtual {
  virtual void first_virtual() {}
  void non_virtual() {}
};

void test() {
  void (HasVirtualAndNonVirtual::*null_pmf)() = nullptr;
  void (HasVirtualAndNonVirtual::*virtual_pmf)() =
      &HasVirtualAndNonVirtual::first_virtual;
  void (HasVirtualAndNonVirtual::*non_virtual_pmf)() =
      &HasVirtualAndNonVirtual::non_virtual;
  assert(virtual_pmf != null_pmf);
  assert(non_virtual_pmf != null_pmf);
}
} // namespace p1_null_distinct_from_member_function

// N4950 [conv.mem]/1: a null pointer constant can be converted to a
// pointer-to-member type; two null member pointer values of the same type
// compare equal.
namespace p1_null_values_compare_equal {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*from_nullptr = nullptr;
  int HasTwoInts::*from_zero = 0;
  assert(from_nullptr == from_zero);
  void (HasTwoInts::*function_from_nullptr)() = nullptr;
  void (HasTwoInts::*function_from_zero)() = 0;
  assert(function_from_nullptr == function_from_zero);
}
} // namespace p1_null_values_compare_equal

// N4950 [conv.mem]/2: "pointer to member of B of type T" converts to "pointer
// to member of D of type T"; the result refers to the member in D's instance
// of B.
namespace p2_base_to_derived_data_member {
struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
};

struct InheritsValue : Precedes, DeclaresValue {
  int own;
};

void test() {
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
}
} // namespace p2_base_to_derived_data_member

// N4950 [conv.mem]/2: indirection through the converted pointer with a D
// object is the same as indirecting through the pointer to member of B with
// the B subobject of D.
namespace p2_base_to_derived_member_function {
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

void test() {
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
}
} // namespace p2_base_to_derived_member_function

// N4950 [conv.mem]/2: the null member pointer value is converted to the null
// member pointer value of the destination type.
namespace p2_null_converts_to_null {
struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
  void increment() { ++value; }
};

struct InheritsValue : Precedes, DeclaresValue {};

void test() {
  int DeclaresValue::*base_pm = nullptr;
  int InheritsValue::*derived_pm = base_pm;
  assert(derived_pm == nullptr);
  void (DeclaresValue::*base_pmf)() = nullptr;
  void (InheritsValue::*derived_pmf)() = base_pmf;
  assert(derived_pmf == nullptr);
}
} // namespace p2_null_converts_to_null

int main() {
  p1_null_distinct_from_member_function::test();
  p1_null_values_compare_equal::test();
  p2_base_to_derived_data_member::test();
  p2_base_to_derived_member_function::test();
  p2_null_converts_to_null::test();
  return 0;
}
