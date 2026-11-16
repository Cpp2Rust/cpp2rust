// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/4.1: two declarations that (re)introduce the same
// name do not correspond if either is a using-declarator.
#include <cassert>

static int library_calls;
static int local_calls;

namespace Library {
int describe(int amount) {
  ++library_calls;
  return amount + 10;
}
} // namespace Library

using Library::describe;

int describe(double amount) {
  ++local_calls;
  return static_cast<int>(amount) + 20;
}

int main() {
  assert(describe(1) == 11);
  assert(library_calls == 1);
  assert(local_calls == 0);

  assert(describe(2.0) == 22);
  assert(library_calls == 1);
  assert(local_calls == 1);
  return 0;
}
