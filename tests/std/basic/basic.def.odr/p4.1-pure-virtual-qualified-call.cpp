// no-compile: refcount, unsafe
// The converter drops the body of a pure virtual function and the
// qualification of calls to it.
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/4.1: a pure virtual function is named by an
// id-expression with an explicitly qualified name. HasPureVirtual::f is pure
// but defined; the qualified call runs the base body, not the override.
#include <cassert>

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

int main() {
  OverridesPureVirtual d;
  HasPureVirtual *p = &d;
  p->f();
  assert(derived_calls == 1);
  assert(base_calls == 1);
  d.HasPureVirtual::f();
  assert(derived_calls == 1);
  assert(base_calls == 2);
  return 0;
}
