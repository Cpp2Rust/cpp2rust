// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.reinterpret.cast]/7: converting a pointer to T1 to a pointer to
// T2 and back to its original type yields the original pointer value.
namespace p7_object_pointer_round_trip {
void test() {
  int value = 5;
  int *ptr = &value;
  int **ptr_ptr = &ptr;
  const int **converted = reinterpret_cast<const int **>(ptr_ptr);
  assert(**converted == 5);
  int **back = reinterpret_cast<int **>(converted);
  assert(back == ptr_ptr);
  **back = 7;
  assert(value == 7);
}
} // namespace p7_object_pointer_round_trip

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
  p7_object_pointer_round_trip::test();
  p10_null_converts_to_null::test();
  p10_2_data_member_round_trip::test();
  return 0;
}
