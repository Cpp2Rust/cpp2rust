// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

#include "../../support/lifetime-tracked.h"

// N4950 [stmt.switch]/2: the value of a condition that is an initialized
// declaration is the value of the declared variable; if of class type, it is
// contextually implicitly converted to an integral or enumeration type.
namespace p2_condition_variable_lives_for_whole_statement {
static int matching_case_runs;
static int other_case_runs;

struct ConvertsToInt {
  static inline int dtorCount = 0;

  int value;

  operator int() const { return value; }
  ~ConvertsToInt() { ++dtorCount; }
};

void test() {
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
}
} // namespace p2_condition_variable_lives_for_whole_statement

// N4950 [stmt.switch]/7: switch (init-statement condition) statement is
// equivalent to a block holding the init-statement and the switch, except
// that the init-statement is in the same scope as the condition.
namespace p7_init_statement_variable_lives_for_whole_statement {
static int matching_case_runs;
static int other_case_runs;

void test() {
  LifetimeTracked::reset();
  int selector = 9;
  switch (LifetimeTracked guard; int selector = 2) {
  case 2:
    ++matching_case_runs;
    guard.use();
    assert(guard.uses == 1);
    assert(selector == 2);
    assert(LifetimeTracked::dtorCount == 0);
    break;
  default:
    ++other_case_runs;
    break;
  }
  assert(matching_case_runs == 1);
  assert(other_case_runs == 0);
  assert(LifetimeTracked::ctorCount == 1);
  assert(LifetimeTracked::dtorCount == 1);
  assert(selector == 9);
}
} // namespace p7_init_statement_variable_lives_for_whole_statement

// [dcl.attr.likelihood]: an attribute on a case label keeps the case.
namespace likelihood_attributes_on_cases {
int classify(int x) {
  int r = 0;
  switch (x) {
  case 1:
    [[unlikely]] return -1;
  [[likely]] case 2:
    r = 2;
    break;
  [[unlikely]] case 3:
    r = 3;
    break;
  default:
    break;
  }
  while (x-- > 0) [[unlikely]] {
    r += 10;
  }
  return r;
}

void test() {
  assert(classify(1) == -1);
  assert(classify(2) == 22);
  assert(classify(3) == 33);
  assert(classify(0) == 0);
}
} // namespace likelihood_attributes_on_cases

int main() {
  likelihood_attributes_on_cases::test();
  p2_condition_variable_lives_for_whole_statement::test();
  p7_init_statement_variable_lives_for_whole_statement::test();
  return 0;
}
