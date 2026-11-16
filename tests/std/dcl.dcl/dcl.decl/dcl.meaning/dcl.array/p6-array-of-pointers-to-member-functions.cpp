// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.array]/6: an object of type "array of N U" consists of a
// contiguously allocated set of N subobjects of type U, numbered 0 to N-1.
#include <cassert>

struct HasThreeMemberFunctions {
  int first_calls = 0;
  int second_calls = 0;
  int third_calls = 0;
  void first() { ++first_calls; }
  void second() { ++second_calls; }
  void third() { ++third_calls; }
};

int main() {
  void (HasThreeMemberFunctions::*functions[3])() = {
      &HasThreeMemberFunctions::third, &HasThreeMemberFunctions::first,
      &HasThreeMemberFunctions::second};
  HasThreeMemberFunctions obj;
  (obj.*functions[0])();
  assert(obj.first_calls == 0);
  assert(obj.second_calls == 0);
  assert(obj.third_calls == 1);
  void (HasThreeMemberFunctions::* * element)() = functions;
  ++element;
  assert(element == &functions[1]);
  (obj.**element)();
  assert(obj.first_calls == 1);
  assert(obj.second_calls == 0);
  assert(obj.third_calls == 1);
  (obj.*element[1])();
  assert(obj.first_calls == 1);
  assert(obj.second_calls == 1);
  assert(obj.third_calls == 1);
  return 0;
}
