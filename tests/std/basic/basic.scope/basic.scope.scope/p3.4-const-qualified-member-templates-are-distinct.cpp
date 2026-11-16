// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/3.4: two non-static member function templates
// have corresponding object parameters if the types of their object
// parameters are equivalent. /4.3.2: function templates whose signatures do
// not correspond do not correspond, so both are declared in the class.
#include <cassert>

struct HasConstTemplateOverload {
  int plain_calls = 0;
  mutable int const_calls = 0;

  template <class Value> int get(Value value) {
    ++plain_calls;
    return value + 1;
  }
  template <class Value> int get(Value value) const {
    ++const_calls;
    return value + 2;
  }
};

int main() {
  HasConstTemplateOverload object;
  const HasConstTemplateOverload &view = object;

  assert(object.get(10) == 11);
  assert(object.plain_calls == 1);
  assert(object.const_calls == 0);

  assert(view.get(10) == 12);
  assert(object.plain_calls == 1);
  assert(object.const_calls == 1);
  return 0;
}
