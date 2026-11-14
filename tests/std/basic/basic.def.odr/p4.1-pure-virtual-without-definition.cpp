// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/4.1: a pure virtual function is not named by a call
// that is not explicitly qualified. HasPureVirtual::f is never defined; the
// virtual call runs the override.
#include <cassert>

static int derived_calls;

struct HasPureVirtual {
  virtual void f() = 0;
};

struct OverridesPureVirtual : HasPureVirtual {
  void f() override { ++derived_calls; }
};

int main() {
  OverridesPureVirtual d;
  HasPureVirtual *p = &d;
  p->f();
  assert(derived_calls == 1);
  return 0;
}
