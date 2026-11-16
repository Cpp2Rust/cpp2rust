// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/7: a scope introduced by a template definition is
// considered to be introduced by the instantiated entity and to contain the
// instantiations of the declarations that inhabit it.
#include <cassert>

template <class Tag> int next_id() {
  static int last = 0;
  return ++last;
}

int main() {
  assert(next_id<int>() == 1);
  assert(next_id<int>() == 2);
  assert(next_id<char>() == 1);
  assert(next_id<int>() == 3);
  assert(next_id<char>() == 2);
  return 0;
}
