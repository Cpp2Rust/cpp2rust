// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/2: .* binds a "pointer to member of T" to a glvalue
// of class T or of a class of which T is an unambiguous and accessible base.
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
  assert(obj.*pm == 2);
  obj.*pm = 4;
  assert(obj.value == 4);
  assert(obj.preceding == 1);
  assert(obj.own == 3);
  return 0;
}
