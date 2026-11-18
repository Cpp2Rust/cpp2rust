// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <utility>

// N4950 [utility.as.const]/1: as_const(t) returns t.
namespace p1_as_const {
struct Counter {
  int value;
  int get() { return value + 1; }
  int get() const { return value; }
};

void test() {
  int number = 3;
  const int &ref = std::as_const(number);
  assert(&ref == &number);
  number = 4;
  assert(ref == 4);
  assert(std::as_const(number) == 4);

  int *pointer = &number;
  assert(std::as_const(pointer) == &number);

  Counter counter{5};
  assert(std::as_const(counter).get() == 5);
  assert(counter.get() == 6);
}
} // namespace p1_as_const

int main() {
  p1_as_const::test();
  return 0;
}
