// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.eq]/4.1: two pointers to members that are both the null member
// pointer value compare equal.
namespace p4_1_both_null {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*first_null = nullptr;
  int HasTwoInts::*second_null = nullptr;
  assert(first_null == second_null);
  assert(!(first_null != second_null));
}
} // namespace p4_1_both_null

// N4950 [expr.eq]/4.2: if only one of two pointers to members is the null
// member pointer value, they compare unequal.
namespace p4_2_one_null {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*null_pm = nullptr;
  int HasTwoInts::*first_pm = &HasTwoInts::first;
  assert(first_pm != null_pm);
  assert(!(first_pm == null_pm));
  assert(null_pm != first_pm);
  assert(!(null_pm == first_pm));
}
} // namespace p4_2_one_null

// N4950 [expr.eq]/4.5: pointers to (possibly different) members of the same
// union compare equal.
namespace p4_5_members_of_same_union {
union HasTwoIntMembers {
  int first;
  int second;
};

void test() {
  int HasTwoIntMembers::*first_pm = &HasTwoIntMembers::first;
  int HasTwoIntMembers::*second_pm = &HasTwoIntMembers::second;
  assert(first_pm == second_pm);
  assert(!(first_pm != second_pm));
}
} // namespace p4_5_members_of_same_union

// N4950 [expr.eq]/4.6: two pointers to members compare equal if they would
// refer to the same member of the same most derived object, otherwise they
// compare unequal.
namespace p4_6_data_members {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*first_pm = &HasTwoInts::first;
  int HasTwoInts::*also_first_pm = &HasTwoInts::first;
  int HasTwoInts::*second_pm = &HasTwoInts::second;
  assert(first_pm == also_first_pm);
  assert(!(first_pm != also_first_pm));
  assert(first_pm != second_pm);
  assert(!(first_pm == second_pm));
}
} // namespace p4_6_data_members

int main() {
  p4_1_both_null::test();
  p4_2_one_null::test();
  p4_5_members_of_same_union::test();
  p4_6_data_members::test();
  return 0;
}
