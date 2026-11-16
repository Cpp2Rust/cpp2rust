// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [stmt.while]/2: the variable created in the condition is destroyed
// and created with each iteration of the loop.
#include <cassert>

struct ConvertsToBool {
  static inline int ctorCount = 0;
  static inline int dtorCount = 0;

  int value;

  ConvertsToBool(int value) : value(value) { ++ctorCount; }
  ~ConvertsToBool() { ++dtorCount; }
  explicit operator bool() const { return value != 0; }
};

int main() {
  int remaining = 2;
  int iterations = 0;
  while (ConvertsToBool condition{remaining}) {
    ++iterations;
    assert(condition.value == remaining);
    assert(ConvertsToBool::ctorCount == iterations);
    assert(ConvertsToBool::dtorCount == iterations - 1);
    --remaining;
  }
  assert(iterations == 2);
  assert(ConvertsToBool::ctorCount == 3);
  assert(ConvertsToBool::dtorCount == 3);
  return 0;
}
