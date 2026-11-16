// panic: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/1.4: each compound statement that is not the
// compound-statement of a handler introduces a block scope that includes that
// statement.
#include <cassert>

static int handler_runs;

int main() {
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
  return 0;
}
