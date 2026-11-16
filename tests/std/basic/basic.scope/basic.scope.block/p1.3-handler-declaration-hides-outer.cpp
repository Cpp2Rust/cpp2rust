// panic: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/1.3: each handler introduces a block scope that
// includes that handler.
#include <cassert>

static int handler_runs;

int main() {
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
  return 0;
}
