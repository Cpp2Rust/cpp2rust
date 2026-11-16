// panic: refcount
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.unary.op]/3.1: a qualified-id naming a variant member of some
// class C yields a pointer to member of class C.
namespace p3_1_anonymous_union_member {
struct HasAnonymousUnion {
  int before;
  union {
    int as_int;
    unsigned as_unsigned;
  };
};

void test() {
  int HasAnonymousUnion::*pm = &HasAnonymousUnion::as_int;
  HasAnonymousUnion obj;
  obj.before = 1;
  obj.as_int = 2;
  assert(obj.*pm == 2);
  obj.*pm = 3;
  assert(obj.as_int == 3);
  assert(obj.before == 1);
}
} // namespace p3_1_anonymous_union_member

// N4950 [expr.unary.op]/3.1: for a qualified-id naming a non-static member m
// of some class C, the result has type "pointer to member of class C" and
// designates C::m; C is the class m is a member of, not the class named.
namespace p3_1_member_named_through_derived_class {
struct DeclaresValue {
  int value;
};

struct InheritsValue : DeclaresValue {
  int own;
};

void test() {
  auto pm = &InheritsValue::value;
  DeclaresValue base{3};
  assert(base.*pm == 3);
  InheritsValue derived{{4}, 5};
  assert(derived.*pm == 4);
}
} // namespace p3_1_member_named_through_derived_class

int main() {
  p3_1_anonymous_union_member::test();
  p3_1_member_named_through_derived_class::test();
  return 0;
}
