// no-compile: refcount, unsafe
// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.mptr.oper]/2: .* binds a "pointer to member of T" to a glvalue
// of class T or of a class of which T is an unambiguous and accessible base.
namespace p2_object_of_derived_class {
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
  int DeclaresValue::*pm = &DeclaresValue::value;
  InheritsValue obj;
  obj.preceding = 1;
  obj.value = 2;
  obj.own = 3;
  assert(obj.*pm == 2);
  obj.*pm = 4;
  assert(obj.value == 4);
  assert(obj.preceding == 1);
  assert(obj.own == 3);
}
} // namespace p2_object_of_derived_class

// N4950 [expr.mptr.oper]/3: the first operand of ->* is a "pointer to U" where
// U is T or a class of which T is an unambiguous and accessible base.
namespace p3_arrow_star_pointer_to_derived_class {
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
  int DeclaresValue::*pm = &DeclaresValue::value;
  InheritsValue obj;
  obj.preceding = 1;
  obj.value = 2;
  obj.own = 3;
  InheritsValue *p = &obj;
  assert(p->*pm == 2);
  p->*pm = 4;
  assert(obj.value == 4);
  assert(obj.preceding == 1);
  assert(obj.own == 3);
}
} // namespace p3_arrow_star_pointer_to_derived_class

// N4950 [expr.mptr.oper]/6: if the result of .* or ->* is a function, it can
// be used only as the operand of the function call operator ().
namespace p6_member_function_call {
struct AccumulatesCalls {
  int total = 0;
  int add(int amount) {
    total += amount;
    return total;
  }
};

void test() {
  int (AccumulatesCalls::*pmf)(int) = &AccumulatesCalls::add;
  AccumulatesCalls first;
  AccumulatesCalls second;
  assert((first.*pmf)(3) == 3);
  assert(first.total == 3);
  assert(second.total == 0);
  AccumulatesCalls *p = &second;
  assert((p->*pmf)(4) == 4);
  assert(first.total == 3);
  assert(second.total == 4);
}
} // namespace p6_member_function_call

// N4950 [expr.mptr.oper]/6: if the result of .* or ->* is a function, it can
// be used only as the operand of the function call operator ().
namespace p6_pointer_to_virtual_member_function {
static int base_calls;
static int override_calls;

struct DeclaresVirtual {
  virtual void run() { ++base_calls; }
};

struct OverridesVirtual : DeclaresVirtual {
  void run() override { ++override_calls; }
};

void test() {
  void (DeclaresVirtual::*pmf)() = &DeclaresVirtual::run;
  DeclaresVirtual base;
  OverridesVirtual overrider;
  (base.*pmf)();
  assert(base_calls == 1);
  assert(override_calls == 0);
  DeclaresVirtual &ref = overrider;
  (ref.*pmf)();
  assert(base_calls == 1);
  assert(override_calls == 1);
}
} // namespace p6_pointer_to_virtual_member_function

int main() {
  p2_object_of_derived_class::test();
  p3_arrow_star_pointer_to_derived_class::test();
  p6_member_function_call::test();
  p6_pointer_to_virtual_member_function::test();
  return 0;
}
