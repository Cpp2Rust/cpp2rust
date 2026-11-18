// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

#include "../support/has-constexpr-ctor.h"
#include "../support/lifetime-tracked.h"

// N4950 [basic.def.odr]/3.2: the potential results of a subscripting operation
// with an array operand are those of the operand. Reading arr[1].m does not
// odr-use arr, so [=] does not capture it.
namespace p3_2_subscript {
void test() {
  HasConstexprCtor::reset();
  constexpr HasConstexprCtor arr[2] = {3, 4};
  auto f = [=] { return arr[1].m; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(f() == 4);
  assert(HasConstexprCtor::copyCount == 0);
}
} // namespace p3_2_subscript

// N4950 [basic.def.odr]/3.3: the potential results of E1.E2 naming a non-static
// data member are those of E1. Reading k.m does not odr-use k, so [=] does not
// capture it.
namespace p3_3_member_access {
void test() {
  HasConstexprCtor::reset();
  constexpr HasConstexprCtor k{5};
  auto f = [=] { return k.m; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(f() == 5);
  assert(HasConstexprCtor::copyCount == 0);
}
} // namespace p3_3_member_access

// N4950 [basic.def.odr]/3.7: the potential results of a glvalue conditional
// expression are those of its second and third operands. Neither k1 nor k2 is
// odr-used, so [=] captures neither.
namespace p3_7_conditional {
void test() {
  HasConstexprCtor::reset();
  constexpr HasConstexprCtor k1{7};
  constexpr HasConstexprCtor k2{9};
  auto f = [=](bool b) { return (b ? k1 : k2).m; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(f(true) == 7);
  assert(f(false) == 9);
  assert(HasConstexprCtor::copyCount == 0);
}
} // namespace p3_7_conditional

// N4950 [basic.def.odr]/3.8: the potential results of a comma expression are
// those of its right operand. k is not odr-used, so [=] does not capture it;
// the left operand is still evaluated.
namespace p3_8_comma {
static int left;

void test() {
  HasConstexprCtor::reset();
  constexpr HasConstexprCtor k{5};
  auto f = [=] { return (++left, k).m; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(left == 0);
  assert(f() == 5);
  assert(left == 1);
  assert(HasConstexprCtor::copyCount == 0);
}
} // namespace p3_8_comma

// N4950 [basic.def.odr]/4.1: a pure virtual function is not named by a call
// that is not explicitly qualified. HasPureVirtual::f is never defined; the
// virtual call runs the override.
namespace p4_1_pure_virtual_without_definition {
static int derived_calls;

struct HasPureVirtual {
  virtual void f() = 0;
};

struct OverridesPureVirtual : HasPureVirtual {
  void f() override { ++derived_calls; }
};

void test() {
  OverridesPureVirtual d;
  HasPureVirtual *p = &d;
  p->f();
  assert(derived_calls == 1);
}
} // namespace p4_1_pure_virtual_without_definition

// N4950 [basic.def.odr]/5.1: a reference that is usable in constant expressions
// is not odr-used. [=] therefore does not capture r, and the lambda uses the
// object r refers to.
namespace p5_1_constant_reference_in_lambda {
LifetimeTracked global;

void test() {
  LifetimeTracked::reset();
  constexpr LifetimeTracked &r = global;
  auto f = [=] { r.use(); };
  assert(LifetimeTracked::copyCount == 0);
  f();
  assert(LifetimeTracked::copyCount == 0);
  assert(global.uses == 1);
  assert(LifetimeTracked::ctorCount == 0);
}
} // namespace p5_1_constant_reference_in_lambda

// N4950 [basic.def.odr]/5.1: only a reference that is usable in constant
// expressions is exempt. r refers to a local object, so it is odr-used and [=]
// captures it, which copies the object it refers to.
namespace p5_1_local_reference_is_odr_used {
void test() {
  LifetimeTracked::reset();
  LifetimeTracked t;
  LifetimeTracked &r = t;
  auto f = [=] { r.use(); };
  assert(LifetimeTracked::copyCount == 1);
  f();
  assert(LifetimeTracked::copyCount == 1);
  assert(LifetimeTracked::useCount == 1);
  assert(t.uses == 0);
}
} // namespace p5_1_local_reference_is_odr_used

// N4950 [basic.def.odr]/5.2: the exemption requires the lvalue-to-rvalue
// conversion. Taking the address of the constant n odr-uses it, so [=] captures
// a copy, and the address seen inside the lambda is not that of n.
namespace p5_2_address_of_constant_is_odr_use {
void test() {
  const int n = 5;
  auto f = [=] { return &n; };
  assert(f() != &n);
  assert(*f() == 5);
}
} // namespace p5_2_address_of_constant_is_odr_use

// N4950 [basic.def.odr]/5.2: the exemption covers only expressions of non-class
// type. Copying the constant k as a whole odr-uses it, so [=] captures it.
namespace p5_2_class_object_is_odr_used {
void test() {
  HasConstexprCtor::reset();
  constexpr HasConstexprCtor k{5};
  auto f = [=] {
    HasConstexprCtor c = k;
    return c.m;
  };
  assert(HasConstexprCtor::copyCount == 1);
  assert(f() == 5);
  assert(HasConstexprCtor::copyCount == 2);
}
} // namespace p5_2_class_object_is_odr_used

// N4950 [basic.def.odr]/5.2: a constant variable of non-class type read by
// value is not odr-used, so the lambda uses n with an empty capture list.
namespace p5_2_constant_in_lambda_without_capture {
void test() {
  const int n = 5;
  auto f = [] { return n; };
  assert(f() == 5);
}
} // namespace p5_2_constant_in_lambda_without_capture

// N4950 [basic.def.odr]/5.2: the exemption requires a variable with no mutable
// subobjects. k has one, so reading k.value odr-uses k and [=] captures it.
namespace p5_2_mutable_subobject_is_odr_used {
static int copies;

struct HasMutableMember {
  int value;
  mutable int scratch;

  constexpr HasMutableMember(int v) : value(v), scratch(0) {}
  HasMutableMember(const HasMutableMember &o)
      : value(o.value), scratch(o.scratch) {
    ++copies;
  }
};

void test() {
  constexpr HasMutableMember k{5};
  auto f = [=] { return k.value; };
  assert(copies == 1);
  assert(f() == 5);
  assert(copies == 1);
}
} // namespace p5_2_mutable_subobject_is_odr_used

// N4950 [basic.def.odr]/6: a structured binding is odr-used only if it appears
// as a potentially-evaluated expression. In sizeof it is unevaluated, so [=]
// does not capture a.
namespace p6_unevaluated_binding_needs_no_capture {
struct HasTwoMembers {
  LifetimeTracked a;
  LifetimeTracked b;
};

void test() {
  LifetimeTracked::reset();
  HasTwoMembers p{LifetimeTracked(), LifetimeTracked()};
  auto &[a, b] = p;
  auto f = [=] { return sizeof(a); };
  assert(LifetimeTracked::copyCount == 0);
  assert(f() == sizeof(LifetimeTracked));
  assert(LifetimeTracked::copyCount == 0);
  assert(LifetimeTracked::useCount == 0);
}
} // namespace p6_unevaluated_binding_needs_no_capture

// N4950 [basic.def.odr]/7: *this is odr-used if this appears as a
// potentially-evaluated expression, including through the implicit
// transformation of a member name. [=] captures this, not the member x, and the
// lambda sees later changes to the object.
namespace p7_member_access_captures_this {
struct HasMemberUsedInLambda {
  LifetimeTracked x;
  int n = 1;
  auto makeLambda() {
    return [=] {
      x.use();
      return n;
    };
  }
};

void test() {
  LifetimeTracked::reset();
  HasMemberUsedInLambda c;
  auto f = c.makeLambda();
  assert(LifetimeTracked::copyCount == 0);
  assert(f() == 1);
  assert(c.x.uses == 1);
  c.n = 10;
  assert(f() == 10);
  assert(c.x.uses == 2);
  assert(LifetimeTracked::copyCount == 0);
}
} // namespace p7_member_access_captures_this

// N4950 [basic.def.odr]/7: *this is odr-used only if this appears as a
// potentially-evaluated expression. In sizeof the member name is unevaluated,
// so the lambda needs no capture.
namespace p7_unevaluated_this_needs_no_capture {
struct HasMemberUsedInLambda {
  LifetimeTracked x;
  auto sizer() {
    return [] { return sizeof(x); };
  }
};

void test() {
  LifetimeTracked::reset();
  HasMemberUsedInLambda c;
  auto f = c.sizer();
  assert(f() == sizeof(LifetimeTracked));
  assert(LifetimeTracked::copyCount == 0);
  assert(LifetimeTracked::useCount == 0);
}
} // namespace p7_unevaluated_this_needs_no_capture

// N4950 [basic.def.odr]/8: a function is odr-used if it is named by a
// potentially-evaluated expression. make() is named only in unevaluated
// operands, so it is never defined; noisy() is defined, but an unevaluated call
// does not run it.
namespace p8_unevaluated_call_needs_no_definition {
static int noisy_calls;

LifetimeTracked make();

int noisy() {
  ++noisy_calls;
  return 1;
}

void test() {
  LifetimeTracked::reset();
  assert(sizeof(make()) == sizeof(LifetimeTracked));
  assert(sizeof(noisy()) == sizeof(int));
  assert(!noexcept(make()));
  assert(!noexcept(noisy()));
  decltype(noisy()) n = 3;
  assert(n == 3);
  assert(noisy_calls == 0);
  assert(LifetimeTracked::ctorCount == 0);
  assert(noisy() == 1);
  assert(noisy_calls == 1);
}
} // namespace p8_unevaluated_call_needs_no_definition

// N4950 [basic.def.odr]/9: a destructor is odr-used if it is potentially
// invoked. A new-expression for a single object does not potentially invoke it,
// and the object is never deleted, so ~HasUndefinedDestructor is never defined.
namespace p9_destructor_not_potentially_invoked {
static int t_ctors;

struct HasUndefinedDestructor {
  HasUndefinedDestructor() { ++t_ctors; }
  ~HasUndefinedDestructor();
};

void test() {
  HasUndefinedDestructor *p = new HasUndefinedDestructor;
  assert(p != nullptr);
  assert(t_ctors == 1);
}
} // namespace p9_destructor_not_potentially_invoked

// N4950 [basic.def.odr]/9: a constructor is odr-used only where an
// initialization selects it. No HasUndefinedCopyCtor is ever copied, so its
// copy constructor is never defined.
namespace p9_unused_copy_constructor {
static int ctor_calls;

struct HasUndefinedCopyCtor {
  int value = 3;

  HasUndefinedCopyCtor() { ++ctor_calls; }
  HasUndefinedCopyCtor(const HasUndefinedCopyCtor &);
};

int read(const HasUndefinedCopyCtor &object) { return object.value; }

void test() {
  HasUndefinedCopyCtor a;
  assert(read(a) == 3);
  assert(ctor_calls == 1);
}
} // namespace p9_unused_copy_constructor

// N4950 [basic.def.odr]/9: an assignment operator is odr-used by an
// implicitly-defined assignment operator of another class.
// HasMemberWithUndefinedCopyAssignment is never assigned, so its assignment
// operator is never implicitly defined and
// HasUndefinedCopyAssignment::operator= is never defined.
namespace p9_unused_implicit_assignment {
static int m_ctors;

struct HasUndefinedCopyAssignment {
  HasUndefinedCopyAssignment() { ++m_ctors; }
  HasUndefinedCopyAssignment &operator=(const HasUndefinedCopyAssignment &);
};

struct HasMemberWithUndefinedCopyAssignment {
  HasUndefinedCopyAssignment m;
};

void test() {
  HasMemberWithUndefinedCopyAssignment a;
  HasMemberWithUndefinedCopyAssignment b;
  assert(m_ctors == 2);
}
} // namespace p9_unused_implicit_assignment

// N4950 [basic.def.odr]/11: a definition is required only for functions
// odr-used outside of a discarded statement. undefined() is called only in a
// discarded statement, so it is never defined and never runs.
namespace p11_discarded_statement {
static int else_runs;

LifetimeTracked undefined();

void g() {
  if constexpr (false) {
    undefined().use();
  } else {
    ++else_runs;
  }
}

void test() {
  LifetimeTracked::reset();
  g();
  assert(else_runs == 1);
  assert(LifetimeTracked::ctorCount == 0);
  assert(LifetimeTracked::useCount == 0);
}
} // namespace p11_discarded_statement

// N4950 [basic.def.odr]/11: a definition is required only for functions
// odr-used outside of a discarded statement. In a template the discarded
// branch is not instantiated; undefined() is called only there, so it is never
// defined and never runs.
namespace p11_discarded_statement_in_template {
static int else_runs;

LifetimeTracked undefined();

template <class T> void g(T) {
  if constexpr (sizeof(T) > 1000) {
    undefined().use();
  } else {
    ++else_runs;
  }
}

void test() {
  LifetimeTracked::reset();
  g(1);
  g('c');
  assert(else_runs == 2);
  assert(LifetimeTracked::ctorCount == 0);
  assert(LifetimeTracked::useCount == 0);
}
} // namespace p11_discarded_statement_in_template

// N4950 [basic.def.odr]/11: a definition is required only for variables
// odr-used outside of a discarded statement. undefined is used only in a
// discarded statement, so it is never defined.
namespace p11_undefined_variable_in_discarded_statement {
static int else_runs;

extern LifetimeTracked undefined;

void g() {
  if constexpr (false) {
    undefined.use();
  } else {
    ++else_runs;
  }
}

void test() {
  LifetimeTracked::reset();
  g();
  assert(else_runs == 1);
  assert(LifetimeTracked::ctorCount == 0);
  assert(LifetimeTracked::useCount == 0);
}
} // namespace p11_undefined_variable_in_discarded_statement

// N4950 [basic.def.odr]/12: an inline function needs a reachable definition
// only where it is odr-used outside of a discarded statement. f is used only in
// a discarded statement, so it is never defined and never runs.
namespace p12_inline_undefined_in_discarded_statement {
static int g_calls;

inline void f();

void g() {
  if constexpr (false) {
    f();
  }
  ++g_calls;
}

void test() {
  g();
  assert(g_calls == 1);
}
} // namespace p12_inline_undefined_in_discarded_statement

// N4950 [basic.def.odr]/13, Example 4: X is never defined. Forming, copying and
// comparing pointers to X does not require X to be complete.
namespace p13_pointer_to_incomplete_class {
struct X;
struct X *x1;
X *x2;

X *identity(X *p) { return p; }

void test() {
  assert(x1 == nullptr);
  assert(identity(x2) == x1);
  X **pp = &x1;
  assert(*pp == nullptr);
}
} // namespace p13_pointer_to_incomplete_class

int main() {
  p3_2_subscript::test();
  p3_3_member_access::test();
  p3_7_conditional::test();
  p3_8_comma::test();
  p4_1_pure_virtual_without_definition::test();
  p5_1_constant_reference_in_lambda::test();
  p5_1_local_reference_is_odr_used::test();
  p5_2_address_of_constant_is_odr_use::test();
  p5_2_class_object_is_odr_used::test();
  p5_2_constant_in_lambda_without_capture::test();
  p5_2_mutable_subobject_is_odr_used::test();
  p6_unevaluated_binding_needs_no_capture::test();
  p7_member_access_captures_this::test();
  p7_unevaluated_this_needs_no_capture::test();
  p8_unevaluated_call_needs_no_definition::test();
  p9_destructor_not_potentially_invoked::test();
  p9_unused_copy_constructor::test();
  p9_unused_implicit_assignment::test();
  p11_discarded_statement::test();
  p11_discarded_statement_in_template::test();
  p11_undefined_variable_in_discarded_statement::test();
  p12_inline_undefined_in_discarded_statement::test();
  p13_pointer_to_incomplete_class::test();
  return 0;
}
