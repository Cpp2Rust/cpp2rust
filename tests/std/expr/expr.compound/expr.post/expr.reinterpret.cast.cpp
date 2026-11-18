// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.reinterpret.cast]/7: an object pointer can be explicitly
// converted to an object pointer of a different type.
namespace p7_object_pointer_conversion {
void test() {
  int value = 5;
  int *ptr = &value;
  int **ptr_ptr = &ptr;
  const int *const *converted = reinterpret_cast<const int *const *>(ptr_ptr);
  assert(converted == ptr_ptr);
  assert(*converted == ptr);
  value = 7;
  assert(**converted == 7);
}
} // namespace p7_object_pointer_conversion

// N4950 [expr.reinterpret.cast]/10: the null member pointer value is converted
// to the null member pointer value of the destination type.
namespace p10_null_converts_to_null {
struct HasTwoInts {
  int first;
  int second;
};

struct HasChar {
  char character;
};

void test() {
  int HasTwoInts::*pm = nullptr;
  char HasChar::*converted = reinterpret_cast<char HasChar::*>(pm);
  assert(converted == nullptr);
}
} // namespace p10_null_converts_to_null

// N4950 [expr.reinterpret.cast]/10.2: converting "pointer to data member of X
// of type T1" to "pointer to data member of Y of type T2" (T2 no more strictly
// aligned than T1) and back yields the original value.
namespace p10_2_data_member_round_trip {
struct HasTwoInts {
  int first;
  int second;
};

struct HasChar {
  char character;
};

void test() {
  char HasChar::*converted =
      reinterpret_cast<char HasChar::*>(&HasTwoInts::second);
  int HasTwoInts::*back = reinterpret_cast<int HasTwoInts::*>(converted);
  assert(back == &HasTwoInts::second);
  HasTwoInts obj{3, 4};
  assert(obj.*back == 4);
}
} // namespace p10_2_data_member_round_trip

int main() {
  p7_object_pointer_conversion::test();
  p10_null_converts_to_null::test();
  p10_2_data_member_round_trip::test();
  return 0;
}
