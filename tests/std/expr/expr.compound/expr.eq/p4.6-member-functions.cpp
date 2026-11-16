// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.6: two pointers to members compare equal if they would
// refer to the same member of the same most derived object, otherwise they
// compare unequal.
#include <cassert>

struct HasTwoMemberFunctions {
  void first() {}
  void second() {}
};

int main() {
  void (HasTwoMemberFunctions::*first_pmf)() = &HasTwoMemberFunctions::first;
  void (HasTwoMemberFunctions::*also_first_pmf)() =
      &HasTwoMemberFunctions::first;
  void (HasTwoMemberFunctions::*second_pmf)() = &HasTwoMemberFunctions::second;
  assert(first_pmf == also_first_pmf);
  assert(!(first_pmf != also_first_pmf));
  assert(first_pmf != second_pmf);
  assert(!(first_pmf == second_pmf));
  return 0;
}
