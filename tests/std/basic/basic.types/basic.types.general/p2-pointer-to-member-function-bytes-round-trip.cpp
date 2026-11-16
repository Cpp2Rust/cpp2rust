// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.types.general]/2: the underlying bytes of an object of
// trivially copyable type can be copied into an array of unsigned char; if
// they are copied back, the object holds its original value.
#include <cassert>
#include <cstring>

struct HasTwoMemberFunctions {
  int first_calls = 0;
  int second_calls = 0;
  void first() { ++first_calls; }
  void second() { ++second_calls; }
};

int main() {
  void (HasTwoMemberFunctions::*pmf)() = &HasTwoMemberFunctions::second;
  unsigned char bytes[sizeof(pmf)];
  std::memcpy(bytes, &pmf, sizeof(pmf));
  pmf = &HasTwoMemberFunctions::first;
  std::memcpy(&pmf, bytes, sizeof(pmf));
  assert(pmf == &HasTwoMemberFunctions::second);
  HasTwoMemberFunctions obj;
  (obj.*pmf)();
  assert(obj.first_calls == 0);
  assert(obj.second_calls == 1);

  void (HasTwoMemberFunctions::*null_pmf)() = nullptr;
  std::memcpy(bytes, &null_pmf, sizeof(null_pmf));
  null_pmf = &HasTwoMemberFunctions::first;
  std::memcpy(&null_pmf, bytes, sizeof(null_pmf));
  assert(null_pmf == nullptr);
  return 0;
}
