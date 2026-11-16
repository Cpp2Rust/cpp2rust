// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.pdecl]/2: a name from an outer scope remains visible up
// to the locus of the declaration that hides it.
#include <cassert>

int main() {
  const int length = 2;
  {
    int length[length] = {7, 8};
    assert(sizeof(length) / sizeof(length[0]) == 2);
    assert(length[0] == 7);
    assert(length[1] == 8);
  }
  assert(length == 2);
  return 0;
}
