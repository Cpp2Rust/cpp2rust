// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/2.10: a block-scope function declaration targets
// the enclosing namespace scope but binds its name in its immediate scope.
#include <cassert>

static int function_calls;

int main() {
  int measure = 5;
  {
    int measure();
    assert(measure() == 7);
    assert(function_calls == 1);
  }
  assert(measure == 5);
  return 0;
}

int measure() {
  ++function_calls;
  return 7;
}
