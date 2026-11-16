// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.pdecl]/1: the locus of a declaration that is a declarator
// is immediately after the complete declarator. /2: a name from an outer scope
// remains visible up to the locus of the declaration that hides it.
#include <cassert>

int main() {
  int level = 5;
  {
    int saved = level, level = saved + 1, next = level + 1;
    assert(saved == 5);
    assert(level == 6);
    assert(next == 7);
  }
  assert(level == 5);
  return 0;
}
