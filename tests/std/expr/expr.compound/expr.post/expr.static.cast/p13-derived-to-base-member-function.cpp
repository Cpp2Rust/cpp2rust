// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/13: "pointer to member of D of type T" can be
// converted to "pointer to member of B of type T", B a base class of D; the
// result points to the original member.
#include <cassert>

struct LacksIncrement {
  int own;
};

struct AddsIncrement : LacksIncrement {
  int value;
  void increment() { ++value; }
};

int main() {
  void (LacksIncrement::*pmf)() =
      static_cast<void (LacksIncrement::*)()>(&AddsIncrement::increment);
  AddsIncrement obj;
  obj.own = 1;
  obj.value = 2;
  LacksIncrement &base = obj;
  (base.*pmf)();
  assert(obj.value == 3);
  assert(obj.own == 1);
  return 0;
}
