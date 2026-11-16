// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/3.2: two non-static member functions have
// corresponding object parameters if their object parameters have the same
// type. /4.3.1: functions whose object parameters do not correspond do not
// correspond, so both are declared in the class.
#include <cassert>

struct HasConstOverload {
  int plain_calls = 0;
  mutable int const_calls = 0;

  int get() {
    ++plain_calls;
    return 1;
  }
  int get() const {
    ++const_calls;
    return 2;
  }
};

int main() {
  HasConstOverload object;
  const HasConstOverload &view = object;

  assert(object.get() == 1);
  assert(object.plain_calls == 1);
  assert(object.const_calls == 0);

  assert(view.get() == 2);
  assert(object.plain_calls == 1);
  assert(object.const_calls == 1);
  return 0;
}
