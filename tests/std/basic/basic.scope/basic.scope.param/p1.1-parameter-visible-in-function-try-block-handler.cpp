// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.param]/1.1: the scope of a function's parameters extends
// to the end of the function-definition.
#include <cassert>

static int body_runs;
static int handler_runs;

int recover(int fallback) try {
  ++body_runs;
  throw 8;
} catch (int thrown) {
  ++handler_runs;
  return fallback + thrown;
}

int main() {
  assert(recover(30) == 38);
  assert(body_runs == 1);
  assert(handler_runs == 1);
  return 0;
}
