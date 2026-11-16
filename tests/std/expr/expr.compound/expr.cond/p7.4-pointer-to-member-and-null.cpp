// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.cond]/7.4: if one or both of the second and third operands have
// pointer-to-member type, they are brought to their composite pointer type.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

static int HasTwoInts::*second_or_null(bool choose_second) {
  return choose_second ? &HasTwoInts::second : nullptr;
}

int main() {
  HasTwoInts obj{3, 4};
  assert(second_or_null(false) == nullptr);
  assert(second_or_null(true) == &HasTwoInts::second);
  assert(obj.*second_or_null(true) == 4);
  return 0;
}
