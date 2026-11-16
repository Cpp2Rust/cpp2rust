// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.reinterpret.cast]/10.2: converting "pointer to data member of X
// of type T1" to "pointer to data member of Y of type T2" (T2 no more strictly
// aligned than T1) and back yields the original value.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

struct HasChar {
  char character;
};

int main() {
  char HasChar::*converted =
      reinterpret_cast<char HasChar::*>(&HasTwoInts::second);
  int HasTwoInts::*back = reinterpret_cast<int HasTwoInts::*>(converted);
  assert(back == &HasTwoInts::second);
  HasTwoInts obj{3, 4};
  assert(obj.*back == 4);
  return 0;
}
