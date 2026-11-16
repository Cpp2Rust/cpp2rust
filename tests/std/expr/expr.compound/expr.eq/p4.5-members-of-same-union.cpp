// translation-fail: refcount
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.5: pointers to (possibly different) members of the same
// union compare equal.
#include <cassert>

union HasTwoIntMembers {
  int first;
  int second;
};

int main() {
  int HasTwoIntMembers::*first_pm = &HasTwoIntMembers::first;
  int HasTwoIntMembers::*second_pm = &HasTwoIntMembers::second;
  assert(first_pm == second_pm);
  assert(!(first_pm != second_pm));
  return 0;
}
