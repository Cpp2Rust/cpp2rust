// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [stmt.switch]/2: the value of a condition that is an initialized
// declaration is the value of the declared variable; if of class type, it is
// contextually implicitly converted to an integral or enumeration type.
#include <cassert>

static int matching_case_runs;
static int other_case_runs;

struct ConvertsToInt {
  static inline int dtorCount = 0;

  int value;

  operator int() const { return value; }
  ~ConvertsToInt() { ++dtorCount; }
};

int main() {
  switch (ConvertsToInt selector{2}) {
  case 2:
    ++matching_case_runs;
    assert(selector.value == 2);
    assert(ConvertsToInt::dtorCount == 0);
    break;
  default:
    ++other_case_runs;
    break;
  }
  assert(matching_case_runs == 1);
  assert(other_case_runs == 0);
  assert(ConvertsToInt::dtorCount == 1);
  return 0;
}
