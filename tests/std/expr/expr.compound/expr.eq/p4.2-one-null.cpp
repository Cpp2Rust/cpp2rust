// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.2: if only one of two pointers to members is the null
// member pointer value, they compare unequal.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*null_pm = nullptr;
  int HasTwoInts::*first_pm = &HasTwoInts::first;
  assert(first_pm != null_pm);
  assert(!(first_pm == null_pm));
  assert(null_pm != first_pm);
  assert(!(null_pm == first_pm));
  return 0;
}
