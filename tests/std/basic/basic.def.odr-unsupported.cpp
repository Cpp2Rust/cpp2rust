// XFAIL: refcount, unsafe
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

#include "../support/has-constexpr-ctor.h"
#include "../support/lifetime-tracked.h"

// N4950 [basic.def.odr]/3.5: the potential results of E1.*E2 are those of E1.
// Reading k.*pm does not odr-use k, so [=] does not capture it.
namespace p3_5_pointer_to_member {
void test() {
  HasConstexprCtor::reset();
  constexpr HasConstexprCtor k{5};
  constexpr int HasConstexprCtor::*pm = &HasConstexprCtor::m;
  auto f = [=] { return k.*pm; };
  assert(HasConstexprCtor::copyCount == 0);
  assert(f() == 5);
  assert(HasConstexprCtor::copyCount == 0);
}
} // namespace p3_5_pointer_to_member

// N4950 [basic.def.odr]/4.1: forming a pointer to member does not name a pure
// virtual function. HasPureVirtual::f is never defined; the call through the
// pointer to member runs the override.
namespace p4_1_pure_virtual_member_pointer {
static int derived_calls;

struct HasPureVirtual {
  virtual void f() = 0;
};

struct OverridesPureVirtual : HasPureVirtual {
  void f() override { ++derived_calls; }
};

void test() {
  void (HasPureVirtual::*pm)() = &HasPureVirtual::f;
  OverridesPureVirtual d;
  HasPureVirtual *p = &d;
  (p->*pm)();
  assert(derived_calls == 1);
}
} // namespace p4_1_pure_virtual_member_pointer

// The converter drops the body of a pure virtual function and the
// qualification of calls to it.
// N4950 [basic.def.odr]/4.1: a pure virtual function is named by an
// id-expression with an explicitly qualified name. HasPureVirtual::f is pure
// but defined; the qualified call runs the base body, not the override.
namespace p4_1_pure_virtual_qualified_call {
static int base_calls, derived_calls;

struct HasPureVirtual {
  virtual void f() = 0;
};

void HasPureVirtual::f() { ++base_calls; }

struct OverridesPureVirtual : HasPureVirtual {
  void f() override {
    ++derived_calls;
    HasPureVirtual::f();
  }
};

void test() {
  OverridesPureVirtual d;
  HasPureVirtual *p = &d;
  p->f();
  assert(derived_calls == 1);
  assert(base_calls == 1);
  d.HasPureVirtual::f();
  assert(derived_calls == 1);
  assert(base_calls == 2);
}
} // namespace p4_1_pure_virtual_qualified_call

// Inside the lambda the converter refers to the original structured binding,
// not to the captured copy.
// N4950 [basic.def.odr]/6: a structured binding is odr-used if it appears as a
// potentially-evaluated expression, so [=] captures a by copy.
namespace p6_evaluated_binding_is_captured {
struct HasTwoMembers {
  LifetimeTracked a;
  LifetimeTracked b;
};

void test() {
  LifetimeTracked::reset();
  HasTwoMembers p{LifetimeTracked(), LifetimeTracked()};
  assert(LifetimeTracked::ctorCount == 2);
  auto &[a, b] = p;
  assert(LifetimeTracked::copyCount == 0);
  auto f = [=] { a.use(); };
  assert(LifetimeTracked::copyCount == 1);
  f();
  assert(LifetimeTracked::copyCount == 1);
  assert(LifetimeTracked::useCount == 1);
  assert(p.a.uses == 0);
}
} // namespace p6_evaluated_binding_is_captured

int main() {
  p3_5_pointer_to_member::test();
  p4_1_pure_virtual_member_pointer::test();
  p4_1_pure_virtual_qualified_call::test();
  p6_evaluated_binding_is_captured::test();
  return 0;
}
