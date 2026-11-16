// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.1: two pointers to members that are both the null member
// pointer value compare equal.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*first_null = nullptr;
  int HasTwoInts::*second_null = nullptr;
  assert(first_null == second_null);
  assert(!(first_null != second_null));
  return 0;
}
