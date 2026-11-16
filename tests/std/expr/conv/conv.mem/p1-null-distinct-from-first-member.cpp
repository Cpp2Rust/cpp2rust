// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [conv.mem]/1: the null member pointer value is distinguishable from
// any pointer to member not created from a null pointer constant.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*null_pm = nullptr;
  int HasTwoInts::*first_pm = &HasTwoInts::first;
  int HasTwoInts::*second_pm = &HasTwoInts::second;
  assert(first_pm != null_pm);
  assert(second_pm != null_pm);
  return 0;
}
