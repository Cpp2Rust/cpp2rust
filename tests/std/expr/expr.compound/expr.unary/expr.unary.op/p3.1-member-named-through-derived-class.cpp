// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.unary.op]/3.1: for a qualified-id naming a non-static member m
// of some class C, the result has type "pointer to member of class C" and
// designates C::m; C is the class m is a member of, not the class named.
#include <cassert>

struct DeclaresValue {
  int value;
};

struct InheritsValue : DeclaresValue {
  int own;
};

int main() {
  auto pm = &InheritsValue::value;
  DeclaresValue base{3};
  assert(base.*pm == 3);
  InheritsValue derived{{4}, 5};
  assert(derived.*pm == 4);
  return 0;
}
