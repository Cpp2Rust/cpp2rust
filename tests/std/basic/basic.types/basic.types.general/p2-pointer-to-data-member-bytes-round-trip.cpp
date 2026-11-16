// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.types.general]/2: the underlying bytes of an object of
// trivially copyable type can be copied into an array of unsigned char; if
// they are copied back, the object holds its original value.
#include <cassert>
#include <cstring>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  unsigned char bytes[sizeof(pm)];
  std::memcpy(bytes, &pm, sizeof(pm));
  pm = &HasTwoInts::first;
  std::memcpy(&pm, bytes, sizeof(pm));
  assert(pm == &HasTwoInts::second);
  HasTwoInts obj{3, 4};
  assert(obj.*pm == 4);

  int HasTwoInts::*null_pm = nullptr;
  std::memcpy(bytes, &null_pm, sizeof(null_pm));
  null_pm = &HasTwoInts::first;
  std::memcpy(&null_pm, bytes, sizeof(null_pm));
  assert(null_pm == nullptr);
  return 0;
}
