// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/3.2: two non-static member functions have
// corresponding object parameters if their object parameters have the same
// type. An explicit object parameter of type X& and the implicit object
// parameter of a &&-qualified member have different types.
#include <cassert>

struct HasExplicitObjectOverload {
  static inline int explicit_calls = 0;
  static inline int rvalue_calls = 0;

  int base = 10;

  int pick(this HasExplicitObjectOverload &self, int amount) {
    ++explicit_calls;
    return self.base + amount;
  }
  int pick(int amount) && {
    ++rvalue_calls;
    return base + amount + 100;
  }
};

int main() {
  HasExplicitObjectOverload object;

  assert(object.pick(1) == 11);
  assert(HasExplicitObjectOverload::explicit_calls == 1);
  assert(HasExplicitObjectOverload::rvalue_calls == 0);

  assert(HasExplicitObjectOverload().pick(2) == 112);
  assert(HasExplicitObjectOverload::explicit_calls == 1);
  assert(HasExplicitObjectOverload::rvalue_calls == 1);
  return 0;
}
