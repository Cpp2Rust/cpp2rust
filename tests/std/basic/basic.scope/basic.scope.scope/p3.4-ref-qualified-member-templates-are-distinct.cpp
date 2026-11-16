// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/3.4: two non-static member function templates
// have corresponding object parameters if the types of their object
// parameters are equivalent. /4.3.2: function templates whose signatures do
// not correspond do not correspond, so both are declared in the class.
#include <cassert>
#include <utility>

struct HasRefQualifiedTemplateOverloads {
  static inline int lvalue_calls = 0;
  static inline int rvalue_calls = 0;

  template <class Value> int get(Value value) & {
    ++lvalue_calls;
    return value + 3;
  }
  template <class Value> int get(Value value) && {
    ++rvalue_calls;
    return value + 4;
  }
};

int main() {
  HasRefQualifiedTemplateOverloads object;

  assert(object.get(10) == 13);
  assert(HasRefQualifiedTemplateOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedTemplateOverloads::rvalue_calls == 0);

  assert(HasRefQualifiedTemplateOverloads().get(10) == 14);
  assert(HasRefQualifiedTemplateOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedTemplateOverloads::rvalue_calls == 1);

  assert(std::move(object).get(10) == 14);
  assert(HasRefQualifiedTemplateOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedTemplateOverloads::rvalue_calls == 2);
  return 0;
}
