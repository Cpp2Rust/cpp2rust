// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [temp.type]/2.6: two values of pointer-to-member type are
// template-argument-equivalent if they refer to the same class member or are
// both the null member pointer value.
namespace p2_6_pointer_to_member_argument {
struct HasTwoInts {
  int first;
  int second;
};

template <int HasTwoInts::*Member> struct CountsPerMember {
  static inline int count = 0;
};

void test() {
  ++CountsPerMember<&HasTwoInts::first>::count;
  assert(CountsPerMember<&HasTwoInts::first>::count == 1);
  assert(CountsPerMember<&HasTwoInts::second>::count == 0);
  assert(CountsPerMember<nullptr>::count == 0);
  CountsPerMember<&HasTwoInts::second>::count += 2;
  assert(CountsPerMember<&HasTwoInts::first>::count == 1);
  assert(CountsPerMember<&HasTwoInts::second>::count == 2);
  assert(CountsPerMember<nullptr>::count == 0);
  CountsPerMember<nullptr>::count += 3;
  assert(CountsPerMember<&HasTwoInts::first>::count == 1);
  assert(CountsPerMember<&HasTwoInts::second>::count == 2);
  assert(CountsPerMember<nullptr>::count == 3);
}
} // namespace p2_6_pointer_to_member_argument

int main() {
  p2_6_pointer_to_member_argument::test();
  return 0;
}
