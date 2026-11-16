// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.6: two pointers to members compare equal if they would
// refer to the same member of the same subobject, otherwise they compare
// unequal.
#include <cassert>

struct RepeatedBase {
  int value;
  void increment() { ++value; }
};

struct FirstPath : RepeatedBase {};

struct SecondPath : RepeatedBase {};

struct JoinsPaths : FirstPath, SecondPath {};

int main() {
  void (RepeatedBase::*base_pmf)() = &RepeatedBase::increment;
  void (FirstPath::*first_pmf)() = base_pmf;
  void (SecondPath::*second_pmf)() = base_pmf;
  void (JoinsPaths::*through_first)() = first_pmf;
  void (JoinsPaths::*through_second)() = second_pmf;
  assert(base_pmf == first_pmf);
  assert(through_first != through_second);
  assert(!(through_first == through_second));
  JoinsPaths obj;
  obj.FirstPath::value = 3;
  obj.SecondPath::value = 5;
  (obj.*through_first)();
  assert(obj.FirstPath::value == 4);
  assert(obj.SecondPath::value == 5);
  (obj.*through_second)();
  assert(obj.FirstPath::value == 4);
  assert(obj.SecondPath::value == 6);
  return 0;
}
