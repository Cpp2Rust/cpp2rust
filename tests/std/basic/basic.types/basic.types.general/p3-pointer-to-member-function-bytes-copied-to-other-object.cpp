// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.types.general]/3: if the underlying bytes of obj1 of trivially
// copyable type are copied into obj2, obj2 holds the same value as obj1.
#include <cassert>
#include <cstring>

struct HasTwoMemberFunctions {
  int first_calls = 0;
  int second_calls = 0;
  void first() { ++first_calls; }
  void second() { ++second_calls; }
};

int main() {
  void (HasTwoMemberFunctions::*source)() = &HasTwoMemberFunctions::second;
  void (HasTwoMemberFunctions::*destination)() = &HasTwoMemberFunctions::first;
  std::memcpy(&destination, &source, sizeof(source));
  assert(destination == source);
  HasTwoMemberFunctions obj;
  (obj.*destination)();
  assert(obj.first_calls == 0);
  assert(obj.second_calls == 1);

  void (HasTwoMemberFunctions::*null_source)() = nullptr;
  std::memcpy(&destination, &null_source, sizeof(null_source));
  assert(destination == nullptr);
  return 0;
}
