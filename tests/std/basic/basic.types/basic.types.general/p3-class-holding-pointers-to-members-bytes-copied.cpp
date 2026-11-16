// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.types.general]/3: if the underlying bytes of obj1 of trivially
// copyable type are copied into obj2, obj2 holds the same value as obj1.
#include <cassert>
#include <cstring>

struct HasValueAndIncrement {
  int padding;
  int value;
  void increment() { ++value; }
};

struct HoldsPointersToMembers {
  int HasValueAndIncrement::*data;
  void (HasValueAndIncrement::*function)();
};

int main() {
  HoldsPointersToMembers source{&HasValueAndIncrement::value,
                                &HasValueAndIncrement::increment};
  HoldsPointersToMembers destination{nullptr, nullptr};
  std::memcpy(&destination, &source, sizeof(source));
  assert(destination.data == &HasValueAndIncrement::value);
  assert(destination.function == &HasValueAndIncrement::increment);
  HasValueAndIncrement obj{1, 2};
  (obj.*destination.function)();
  assert(obj.*destination.data == 3);
  assert(obj.padding == 1);
  return 0;
}
