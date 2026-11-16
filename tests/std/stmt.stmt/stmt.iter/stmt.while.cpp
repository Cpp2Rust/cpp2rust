// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [stmt.while]/2: the variable created in the condition is destroyed
// and created with each iteration of the loop.
namespace p2_condition_variable_created_each_iteration {
struct ConvertsToBool {
  static inline int ctorCount = 0;
  static inline int dtorCount = 0;

  int value;

  ConvertsToBool(int value) : value(value) { ++ctorCount; }
  ~ConvertsToBool() { ++dtorCount; }
  explicit operator bool() const { return value != 0; }
};

void test() {
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
}
} // namespace p2_condition_variable_created_each_iteration

// N4950 [stmt.while]/2: the variable created in the condition is destroyed
// and created with each iteration of the loop.
namespace p2_continue_destroys_condition_variable {
struct ConvertsToBool {
  static inline int ctorCount = 0;
  static inline int dtorCount = 0;

  int value;

  ConvertsToBool(int value) : value(value) { ++ctorCount; }
  ~ConvertsToBool() { ++dtorCount; }
  explicit operator bool() const { return value != 0; }
};

void test() {
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
}
} // namespace p2_continue_destroys_condition_variable

int main() {
  p2_condition_variable_created_each_iteration::test();
  p2_continue_destroys_condition_variable::test();
  return 0;
}
