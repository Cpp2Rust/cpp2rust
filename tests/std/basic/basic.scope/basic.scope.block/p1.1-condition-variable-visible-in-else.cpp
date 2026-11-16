// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/1.1: each selection or iteration statement
// introduces a block scope that includes that statement.
#include <cassert>

static int then_runs;
static int else_runs;

struct ConvertsToBool {
  static inline int dtorCount = 0;

  int value;

  explicit operator bool() const { return value != 0; }
  ~ConvertsToBool() { ++dtorCount; }
};

int main() {
  if (ConvertsToBool condition{0}) {
    ++then_runs;
  } else {
    ++else_runs;
    assert(condition.value == 0);
    assert(ConvertsToBool::dtorCount == 0);
  }
  assert(then_runs == 0);
  assert(else_runs == 1);
  assert(ConvertsToBool::dtorCount == 1);

  if (ConvertsToBool condition{6}) {
    ++then_runs;
    assert(condition.value == 6);
    assert(ConvertsToBool::dtorCount == 1);
  } else {
    ++else_runs;
  }
  assert(then_runs == 1);
  assert(else_runs == 1);
  assert(ConvertsToBool::dtorCount == 2);
  return 0;
}
