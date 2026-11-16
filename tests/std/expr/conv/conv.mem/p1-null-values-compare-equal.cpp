// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [conv.mem]/1: a null pointer constant can be converted to a
// pointer-to-member type; two null member pointer values of the same type
// compare equal.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*from_nullptr = nullptr;
  int HasTwoInts::*from_zero = 0;
  assert(from_nullptr == from_zero);
  void (HasTwoInts::*function_from_nullptr)() = nullptr;
  void (HasTwoInts::*function_from_zero)() = 0;
  assert(function_from_nullptr == function_from_zero);
  return 0;
}
