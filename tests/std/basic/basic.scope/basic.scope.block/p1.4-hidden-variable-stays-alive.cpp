// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.block]/1.4: each compound statement that is not the
// compound-statement of a handler introduces a block scope that includes that
// statement.
#include <cassert>

int main() {
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
  return 0;
}
