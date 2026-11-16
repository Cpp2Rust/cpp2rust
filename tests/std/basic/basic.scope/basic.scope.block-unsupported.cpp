// panic: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.scope.block]/1.3: each handler introduces a block scope that
// includes that handler.
namespace p1_3_handler_declaration_hides_outer {
static int handler_runs;

void test() {
  int code = 1;
  try {
    throw 2;
  } catch (int code) {
    ++handler_runs;
    assert(code == 2);
    code = 3;
    assert(code == 3);
  }
  assert(handler_runs == 1);
  assert(code == 1);
}
} // namespace p1_3_handler_declaration_hides_outer

// N4950 [basic.scope.block]/1.4: each compound statement that is not the
// compound-statement of a handler introduces a block scope that includes that
// statement.
namespace p1_4_try_block_variable_not_visible_in_handler {
static int handler_runs;

void test() {
  int stage = 10;
  try {
    int stage = 20;
    throw stage;
  } catch (int thrown) {
    ++handler_runs;
    assert(thrown == 20);
    assert(stage == 10);
  }
  assert(handler_runs == 1);
  assert(stage == 10);
}
} // namespace p1_4_try_block_variable_not_visible_in_handler

int main() {
  p1_3_handler_declaration_hides_outer::test();
  p1_4_try_block_variable_not_visible_in_handler::test();
  return 0;
}
