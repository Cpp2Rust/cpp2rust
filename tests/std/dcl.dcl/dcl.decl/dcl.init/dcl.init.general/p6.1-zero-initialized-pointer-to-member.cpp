// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.init.general]/6.1: zero-initializing a scalar converts the
// integer literal 0 to its type; for a pointer to member that is the null
// member pointer value.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

struct HoldsSelector {
  int HasTwoInts::*selector;
};

static int HasTwoInts::*static_pm;

int main() {
  assert(static_pm == nullptr);
  assert(static_pm != &HasTwoInts::first);
  int HasTwoInts::*value_initialized{};
  assert(value_initialized == nullptr);
  assert(value_initialized != &HasTwoInts::first);
  HoldsSelector holder{};
  assert(holder.selector == nullptr);
  assert(holder.selector != &HasTwoInts::first);
  return 0;
}
