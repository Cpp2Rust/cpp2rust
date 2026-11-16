// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/13: "pointer to member of D of type T" can be
// converted to "pointer to member of B of type T", B a base class of D; the
// result points to the original member.
#include <cassert>

struct LacksValue {
  int own;
};

struct AddsValue : LacksValue {
  int value;
};

int main() {
  int LacksValue::*pm = static_cast<int LacksValue::*>(&AddsValue::value);
  AddsValue obj;
  obj.own = 1;
  obj.value = 2;
  LacksValue &base = obj;
  assert(base.*pm == 2);
  base.*pm = 3;
  assert(obj.value == 3);
  assert(obj.own == 1);
  return 0;
}
