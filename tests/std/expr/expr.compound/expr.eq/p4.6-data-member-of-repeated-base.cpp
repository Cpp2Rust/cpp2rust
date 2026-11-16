// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.6: two pointers to members compare equal if they would
// refer to the same member of the same subobject, otherwise they compare
// unequal.
#include <cassert>

struct RepeatedBase {
  int value;
};

struct FirstPath : RepeatedBase {};

struct SecondPath : RepeatedBase {};

struct JoinsPaths : FirstPath, SecondPath {};

int main() {
  int RepeatedBase::*base_pm = &RepeatedBase::value;
  int FirstPath::*first_pm = base_pm;
  int SecondPath::*second_pm = base_pm;
  int JoinsPaths::*through_first = first_pm;
  int JoinsPaths::*through_second = second_pm;
  assert(base_pm == first_pm);
  assert(through_first != through_second);
  assert(!(through_first == through_second));
  JoinsPaths obj;
  obj.FirstPath::value = 3;
  obj.SecondPath::value = 4;
  assert(obj.*through_first == 3);
  assert(obj.*through_second == 4);
  return 0;
}
