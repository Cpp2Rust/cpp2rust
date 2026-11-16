// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/3: the first operand of ->* is a "pointer to U" where
// U is T or a class of which T is an unambiguous and accessible base.
#include <cassert>

struct Precedes {
  int preceding;
};

struct DeclaresValue {
  int value;
};

struct InheritsValue : Precedes, DeclaresValue {
  int own;
};

int main() {
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
  return 0;
}
