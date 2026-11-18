// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [conv.mem]/1: the null member pointer value is distinguishable from
// any pointer to member not created from a null pointer constant.
namespace p1_null_distinct_from_first_member {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*null_pm = nullptr;
  int HasTwoInts::*first_pm = &HasTwoInts::first;
  int HasTwoInts::*second_pm = &HasTwoInts::second;
  assert(first_pm != null_pm);
  assert(second_pm != null_pm);
}
} // namespace p1_null_distinct_from_first_member

int main() {
  p1_null_distinct_from_first_member::test();
  return 0;
}
