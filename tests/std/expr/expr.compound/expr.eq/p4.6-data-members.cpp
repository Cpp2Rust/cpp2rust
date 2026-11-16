// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.6: two pointers to members compare equal if they would
// refer to the same member of the same most derived object, otherwise they
// compare unequal.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*first_pm = &HasTwoInts::first;
  int HasTwoInts::*also_first_pm = &HasTwoInts::first;
  int HasTwoInts::*second_pm = &HasTwoInts::second;
  assert(first_pm == also_first_pm);
  assert(!(first_pm != also_first_pm));
  assert(first_pm != second_pm);
  assert(!(first_pm == second_pm));
  return 0;
}
