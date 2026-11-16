// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/2.10: a block-scope extern declaration targets a
// larger enclosing scope but binds its name in its immediate scope.
#include <cassert>

int level = 11;

int main() {
  int level = 22;
  assert(level == 22);
  {
    extern int level;
    assert(level == 11);
    level = 33;
  }
  assert(level == 22);
  assert(::level == 33);
  return 0;
}
