// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/6: if the result of .* or ->* is a function, it can
// be used only as the operand of the function call operator ().
#include <cassert>

static int base_calls;
static int override_calls;

struct DeclaresVirtual {
  virtual void run() { ++base_calls; }
};

struct OverridesVirtual : DeclaresVirtual {
  void run() override { ++override_calls; }
};

int main() {
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
  return 0;
}
