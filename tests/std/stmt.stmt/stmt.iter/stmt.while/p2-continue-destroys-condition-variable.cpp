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
  int remaining = 3;
  int continued = 0;
  int completed = 0;
  while (ConvertsToBool condition{remaining}) {
    --remaining;
    if (condition.value == 3) {
      ++continued;
      continue;
    }
    assert(ConvertsToBool::ctorCount == 2);
    assert(ConvertsToBool::dtorCount == 1);
    ++completed;
    break;
  }
  assert(continued == 1);
  assert(completed == 1);
  assert(remaining == 1);
  assert(ConvertsToBool::ctorCount == 2);
  assert(ConvertsToBool::dtorCount == 2);
  return 0;
}
