// panic: refcount
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.unary.op]/3.1: a qualified-id naming a variant member of some
// class C yields a pointer to member of class C.
#include <cassert>

struct HasAnonymousUnion {
  int before;
  union {
    int as_int;
    unsigned as_unsigned;
  };
};

int main() {
  int HasAnonymousUnion::*pm = &HasAnonymousUnion::as_int;
  HasAnonymousUnion obj;
  obj.before = 1;
  obj.as_int = 2;
  assert(obj.*pm == 2);
  obj.*pm = 3;
  assert(obj.as_int == 3);
  assert(obj.before == 1);
  return 0;
}
