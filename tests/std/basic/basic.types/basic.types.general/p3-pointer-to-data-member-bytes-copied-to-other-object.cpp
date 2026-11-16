// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.types.general]/3: if the underlying bytes of obj1 of trivially
// copyable type are copied into obj2, obj2 holds the same value as obj1.
#include <cassert>
#include <cstring>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*source = &HasTwoInts::second;
  int HasTwoInts::*destination = &HasTwoInts::first;
  std::memcpy(&destination, &source, sizeof(source));
  assert(destination == source);
  HasTwoInts obj{3, 4};
  assert(obj.*destination == 4);

  int HasTwoInts::*null_source = nullptr;
  std::memcpy(&destination, &null_source, sizeof(null_source));
  assert(destination == nullptr);
  return 0;
}
