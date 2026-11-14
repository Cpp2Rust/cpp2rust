// XFAIL: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/4.1: forming a pointer to member does not name a pure
// virtual function. HasPureVirtual::f is never defined; the call through the
// pointer to member runs the override.
#include <cassert>

static int derived_calls;

struct HasPureVirtual {
  virtual void f() = 0;
};

struct OverridesPureVirtual : HasPureVirtual {
  void f() override { ++derived_calls; }
};

int main() {
  void (HasPureVirtual::*pm)() = &HasPureVirtual::f;
  OverridesPureVirtual d;
  HasPureVirtual *p = &d;
  (p->*pm)();
  assert(derived_calls == 1);
  return 0;
}
