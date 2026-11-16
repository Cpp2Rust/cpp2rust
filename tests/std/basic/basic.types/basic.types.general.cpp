// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstring>

// N4950 [basic.types.general]/2: the underlying bytes of an object of
// trivially copyable type can be copied into an array of unsigned char; if
// they are copied back, the object holds its original value.
namespace p2_pointer_to_data_member_bytes_round_trip {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
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
}
} // namespace p2_pointer_to_data_member_bytes_round_trip

// N4950 [basic.types.general]/3: if the underlying bytes of obj1 of trivially
// copyable type are copied into obj2, obj2 holds the same value as obj1.
namespace p3_pointer_to_data_member_bytes_copied_to_other_object {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*source = &HasTwoInts::second;
  int HasTwoInts::*destination = &HasTwoInts::first;
  std::memcpy(&destination, &source, sizeof(source));
  assert(destination == source);
  HasTwoInts obj{3, 4};
  assert(obj.*destination == 4);

  int HasTwoInts::*null_source = nullptr;
  std::memcpy(&destination, &null_source, sizeof(null_source));
  assert(destination == nullptr);
}
} // namespace p3_pointer_to_data_member_bytes_copied_to_other_object

int main() {
  p2_pointer_to_data_member_bytes_round_trip::test();
  p3_pointer_to_data_member_bytes_copied_to_other_object::test();
  return 0;
}
