// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

#include "../../support/lifetime-tracked.h"

// N4950 [basic.scope.block]/1.1: each selection or iteration statement
// introduces a block scope that includes that statement.
namespace p1_1_condition_variable_visible_in_else {
static int then_runs;
static int else_runs;

struct ConvertsToBool {
  static inline int dtorCount = 0;

  int value;

  explicit operator bool() const { return value != 0; }
  ~ConvertsToBool() { ++dtorCount; }
};

void test() {
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
}
} // namespace p1_1_condition_variable_visible_in_else

// N4950 [basic.scope.block]/1.1: each selection or iteration statement
// introduces a block scope that includes that statement.
namespace p1_1_for_init_variable_hides_outer {
void test() {
  int index = 42;
  int iterations = 0;
  int sum = 0;
  for (int index = 0; index < 3; ++index) {
    ++iterations;
    sum += index;
  }
  assert(iterations == 3);
  assert(sum == 3);
  assert(index == 42);
}
} // namespace p1_1_for_init_variable_hides_outer

// N4950 [basic.scope.block]/1.2: each substatement of a selection or iteration
// statement introduces a block scope that includes that substatement.
namespace p1_2_unbraced_substatement_is_a_scope {
void test() {
  LifetimeTracked::reset();
  LifetimeTracked scoped;
  assert(LifetimeTracked::ctorCount == 1);

  for (int index = 0; index < 3; ++index)
    LifetimeTracked scoped;
  assert(LifetimeTracked::ctorCount == 4);
  assert(LifetimeTracked::dtorCount == 3);

  if (LifetimeTracked::ctorCount == 4)
    LifetimeTracked scoped;
  assert(LifetimeTracked::ctorCount == 5);
  assert(LifetimeTracked::dtorCount == 4);

  if (LifetimeTracked::ctorCount == 0)
    ;
  else
    LifetimeTracked scoped;
  assert(LifetimeTracked::ctorCount == 6);
  assert(LifetimeTracked::dtorCount == 5);

  scoped.use();
  assert(scoped.uses == 1);
}
} // namespace p1_2_unbraced_substatement_is_a_scope

// N4950 [basic.scope.block]/1.4: each compound statement that is not the
// compound-statement of a handler introduces a block scope that includes that
// statement.
namespace p1_4_hidden_variable_stays_alive {
void test() {
  int depth = 1;
  int *outermost = &depth;
  {
    int depth = 2;
    *outermost += 10;
    assert(depth == 2);
    {
      int depth = 3;
      *outermost += 100;
      assert(depth == 3);
    }
    assert(depth == 2);
  }
  assert(depth == 111);
}
} // namespace p1_4_hidden_variable_stays_alive

// N4950 [basic.scope.block]/1.4: each compound statement that is not the
// compound-statement of a handler introduces a block scope that includes that
// statement.
namespace p1_4_nested_block_hides_and_restores {
void test() {
  LifetimeTracked::reset();
  LifetimeTracked tracked;
  {
    LifetimeTracked tracked;
    assert(LifetimeTracked::ctorCount == 2);
    tracked.use();
    assert(tracked.uses == 1);
    assert(LifetimeTracked::dtorCount == 0);
  }
  assert(LifetimeTracked::dtorCount == 1);
  assert(tracked.uses == 0);

  tracked.use();
  assert(tracked.uses == 1);
  assert(LifetimeTracked::useCount == 2);
}
} // namespace p1_4_nested_block_hides_and_restores

// N4950 [basic.scope.block]/2.2: a declaration in the block scope of a
// substatement must not conflict with one in the parent scope, unless the
// substatement is itself a selection or iteration statement.
namespace p2_2_nested_iteration_statement_redeclares_loop_variable {
void test() {
  int outer_iterations = 0;
  int inner_iterations = 0;
  int sum = 0;
  for (int index = 0; index < 2; ++index, ++outer_iterations)
    for (int index = 10; index < 13; ++index) {
      ++inner_iterations;
      sum += index;
    }
  assert(outer_iterations == 2);
  assert(inner_iterations == 6);
  assert(sum == 66);
}
} // namespace p2_2_nested_iteration_statement_redeclares_loop_variable

// N4950 [basic.scope.block]/2.2: a declaration in the block scope of a
// substatement must not conflict with one in the parent scope, unless the
// substatement is itself a selection or iteration statement.
namespace p2_2_nested_selection_statement_redeclares_condition_variable {
static int inner_then_runs;
static int inner_else_runs;
static int outer_else_runs;

void test() {
  if (int choice = 4)
    if (int choice = 0) {
      ++inner_then_runs;
    } else {
      ++inner_else_runs;
      assert(choice == 0);
    }
  else {
    ++outer_else_runs;
    assert(choice == 0);
  }
  assert(inner_then_runs == 0);
  assert(inner_else_runs == 1);
  assert(outer_else_runs == 0);
}
} // namespace p2_2_nested_selection_statement_redeclares_condition_variable

int main() {
  p1_1_condition_variable_visible_in_else::test();
  p1_1_for_init_variable_hides_outer::test();
  p1_2_unbraced_substatement_is_a_scope::test();
  p1_4_hidden_variable_stays_alive::test();
  p1_4_nested_block_hides_and_restores::test();
  p2_2_nested_iteration_statement_redeclares_loop_variable::test();
  p2_2_nested_selection_statement_redeclares_condition_variable::test();
  return 0;
}
