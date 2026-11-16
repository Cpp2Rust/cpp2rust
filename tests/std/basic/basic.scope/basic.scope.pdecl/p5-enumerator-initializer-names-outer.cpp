// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.pdecl]/5: the locus of an enumerator-definition is
// immediately after it.
#include <cassert>

int main() {
  const int limit = 12;
  {
    enum { limit = limit + 1, above = limit + 1 };
    assert(limit == 13);
    assert(above == 14);
  }
  assert(limit == 12);
  return 0;
}
