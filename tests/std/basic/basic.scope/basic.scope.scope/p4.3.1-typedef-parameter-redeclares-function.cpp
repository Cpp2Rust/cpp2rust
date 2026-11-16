// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/4.3.1: two function declarations correspond when
// they have the same non-object-parameter-type-list; otherwise each declares
// its own function.
#include <cassert>

static int int_calls;
static int unit_calls;

typedef int Count;
enum Unit : int { single = 1 };

int describe(int amount);

int describe(Unit unit) {
  ++unit_calls;
  return unit + 20;
}

int main() {
  assert(describe(3) == 13);
  assert(int_calls == 1);
  assert(unit_calls == 0);

  assert(describe(single) == 21);
  assert(int_calls == 1);
  assert(unit_calls == 1);
  return 0;
}

int describe(Count amount) {
  ++int_calls;
  return amount + 10;
}
