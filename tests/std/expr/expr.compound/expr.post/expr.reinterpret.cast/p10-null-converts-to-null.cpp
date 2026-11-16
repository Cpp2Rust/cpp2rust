// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.reinterpret.cast]/10: the null member pointer value is converted
// to the null member pointer value of the destination type.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

struct HasChar {
  char character;
};

int main() {
  int HasTwoInts::*pm = nullptr;
  char HasChar::*converted = reinterpret_cast<char HasChar::*>(pm);
  assert(converted == nullptr);
  return 0;
}
