// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.eq]/4.6: two pointers to members compare equal if they would
// refer to the same member of the same subobject, otherwise they compare
// unequal.
namespace p4_6_data_member_of_repeated_base {
struct RepeatedBase {
  int value;
};

struct FirstPath : RepeatedBase {};

struct SecondPath : RepeatedBase {};

struct JoinsPaths : FirstPath, SecondPath {};

void test() {
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
}
} // namespace p4_6_data_member_of_repeated_base

// N4950 [expr.eq]/4.6: two pointers to members compare equal if they would
// refer to the same member of the same subobject, otherwise they compare
// unequal.
namespace p4_6_member_function_of_repeated_base {
struct RepeatedBase {
  int value;
  void increment() { ++value; }
};

struct FirstPath : RepeatedBase {};

struct SecondPath : RepeatedBase {};

struct JoinsPaths : FirstPath, SecondPath {};

void test() {
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
}
} // namespace p4_6_member_function_of_repeated_base

// N4950 [expr.eq]/4.6: two pointers to members compare equal if they would
// refer to the same member of the same most derived object, otherwise they
// compare unequal.
namespace p4_6_member_functions {
struct HasTwoMemberFunctions {
  void first() {}
  void second() {}
};

void test() {
  void (HasTwoMemberFunctions::*first_pmf)() = &HasTwoMemberFunctions::first;
  void (HasTwoMemberFunctions::*also_first_pmf)() =
      &HasTwoMemberFunctions::first;
  void (HasTwoMemberFunctions::*second_pmf)() = &HasTwoMemberFunctions::second;
  assert(first_pmf == also_first_pmf);
  assert(!(first_pmf != also_first_pmf));
  assert(first_pmf != second_pmf);
  assert(!(first_pmf == second_pmf));
}
} // namespace p4_6_member_functions

int main() {
  p4_6_data_member_of_repeated_base::test();
  p4_6_member_function_of_repeated_base::test();
  p4_6_member_functions::test();
  return 0;
}
