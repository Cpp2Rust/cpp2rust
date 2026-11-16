// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.scope.param]/1.1: the scope of a function's parameters extends
// to the end of the function-definition.
namespace p1_1_parameter_visible_in_function_try_block_handler {
static int body_runs;
static int handler_runs;

int recover(int fallback) try {
  ++body_runs;
  throw 8;
} catch (int thrown) {
  ++handler_runs;
  return fallback + thrown;
}

void test() {
  assert(recover(30) == 38);
  assert(body_runs == 1);
  assert(handler_runs == 1);
}
} // namespace p1_1_parameter_visible_in_function_try_block_handler

int main() {
  p1_1_parameter_visible_in_function_try_block_handler::test();
  return 0;
}
