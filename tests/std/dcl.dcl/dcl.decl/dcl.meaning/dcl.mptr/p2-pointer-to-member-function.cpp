// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.mptr]/2: "T (C::*)(parameters)" declares a pointer to member of
// class C of function type.
#include <cassert>

struct HasTwoMemberFunctions {
  int first_total = 0;
  int second_total = 0;
  void first(int amount) { first_total += amount; }
  void second(int amount) { second_total += amount; }
};

int main() {
  void (HasTwoMemberFunctions::*pmf)(int) = &HasTwoMemberFunctions::first;
  HasTwoMemberFunctions obj;
  (obj.*pmf)(3);
  assert(obj.first_total == 3);
  assert(obj.second_total == 0);
  pmf = &HasTwoMemberFunctions::second;
  (obj.*pmf)(4);
  assert(obj.first_total == 3);
  assert(obj.second_total == 4);
  return 0;
}
