// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

#include "../../support/lifetime-tracked.h"

// N4950 [stmt.if]/3: if (init-statement condition) statement else statement
// is equivalent to a block holding the init-statement and the if, except that
// the init-statement is in the same scope as the condition.
namespace p3_init_statement_variable_lives_for_whole_statement {
static int then_runs;
static int else_runs;

void test() {
  LifetimeTracked::reset();
  int choice = 9;
  if (LifetimeTracked guard; int choice = 0) {
    ++then_runs;
  } else {
    ++else_runs;
    guard.use();
    assert(guard.uses == 1);
    assert(choice == 0);
    assert(LifetimeTracked::dtorCount == 0);
  }
  assert(then_runs == 0);
  assert(else_runs == 1);
  assert(LifetimeTracked::dtorCount == 1);

  if (LifetimeTracked guard; int choice = 4) {
    ++then_runs;
    guard.use();
    assert(guard.uses == 1);
    assert(choice == 4);
    assert(LifetimeTracked::dtorCount == 1);
  }
  assert(then_runs == 1);
  assert(LifetimeTracked::ctorCount == 2);
  assert(LifetimeTracked::dtorCount == 2);
  assert(choice == 9);
}
} // namespace p3_init_statement_variable_lives_for_whole_statement

// N4950 [stmt.if]/5: If a consteval if statement is evaluated in a context
// that is manifestly constant-evaluated (7.7), the first substatement is
// executed. Otherwise, if the else part of the selection statement is present,
// then the second substatement is executed.
namespace p5_consteval_if_at_run_time_executes_else {
int pick() {
  if consteval {
    return 1;
  } else {
    return 2;
  }
}

int only_then() {
  int r = 3;
  if consteval {
    r = 4;
  }
  return r;
}

void test() {
  assert(pick() == 2);
  assert(only_then() == 3);
}
} // namespace p5_consteval_if_at_run_time_executes_else

// N4950 [stmt.if]/6: An if statement of the form
// if ! consteval compound-statement
// is not itself a consteval if statement, but is equivalent to the consteval if
// statement
// if consteval { } else compound-statement
// An if statement of the form
// if ! consteval compound-statement1 else statement2
// is not itself a consteval if statement, but is equivalent to the consteval if
// statement
// if consteval statement2 else compound-statement1
namespace p6_negated_consteval_if_at_run_time_executes_first {
int pick() {
  if !consteval {
    return 2;
  } else {
    return 1;
  }
}

int only_then() {
  int r = 3;
  if !consteval {
    r = 4;
  }
  return r;
}

void test() {
  assert(pick() == 2);
  assert(only_then() == 4);
}
} // namespace p6_negated_consteval_if_at_run_time_executes_first

int main() {
  p3_init_statement_variable_lives_for_whole_statement::test();
  p5_consteval_if_at_run_time_executes_else::test();
  p6_negated_consteval_if_at_run_time_executes_first::test();
  return 0;
}
