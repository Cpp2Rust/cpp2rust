// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [dcl.mptr]/2: "T (C::*)(parameters)" declares a pointer to member of
// class C of function type.
namespace p2_pointer_to_member_function {
struct HasTwoMemberFunctions {
  int first_total = 0;
  int second_total = 0;
  void first(int amount) { first_total += amount; }
  void second(int amount) { second_total += amount; }
};

void test() {
  void (HasTwoMemberFunctions::*pmf)(int) = &HasTwoMemberFunctions::first;
  HasTwoMemberFunctions obj;
  (obj.*pmf)(3);
  assert(obj.first_total == 3);
  assert(obj.second_total == 0);
  pmf = &HasTwoMemberFunctions::second;
  (obj.*pmf)(4);
  assert(obj.first_total == 3);
  assert(obj.second_total == 4);
}
} // namespace p2_pointer_to_member_function

int main() {
  p2_pointer_to_member_function::test();
  return 0;
}
