// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/9: a destructor is odr-used if it is potentially
// invoked. A new-expression for a single object does not potentially invoke it,
// and the object is never deleted, so ~HasUndefinedDestructor is never defined.
#include <cassert>

static int t_ctors;

struct HasUndefinedDestructor {
  HasUndefinedDestructor() { ++t_ctors; }
  ~HasUndefinedDestructor();
};

int main() {
  HasUndefinedDestructor *p = new HasUndefinedDestructor;
  assert(p != nullptr);
  assert(t_ctors == 1);
  return 0;
}
