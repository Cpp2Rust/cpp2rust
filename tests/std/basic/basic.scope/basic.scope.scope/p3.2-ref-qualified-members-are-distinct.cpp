// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/3.2: two non-static member functions have
// corresponding object parameters if their object parameters have the same
// type. /4.3.1: functions whose object parameters do not correspond do not
// correspond, so both are declared in the class.
#include <cassert>
#include <utility>

struct HasRefQualifiedOverloads {
  static inline int lvalue_calls = 0;
  static inline int rvalue_calls = 0;

  int get() & {
    ++lvalue_calls;
    return 3;
  }
  int get() && {
    ++rvalue_calls;
    return 4;
  }
};

int main() {
  HasRefQualifiedOverloads object;

  assert(object.get() == 3);
  assert(HasRefQualifiedOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedOverloads::rvalue_calls == 0);

  assert(HasRefQualifiedOverloads().get() == 4);
  assert(HasRefQualifiedOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedOverloads::rvalue_calls == 1);

  assert(std::move(object).get() == 4);
  assert(HasRefQualifiedOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedOverloads::rvalue_calls == 2);
  return 0;
}
