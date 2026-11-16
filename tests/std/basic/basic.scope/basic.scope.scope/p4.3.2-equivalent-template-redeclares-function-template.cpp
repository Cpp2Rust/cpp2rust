// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/4.3.2: two function template declarations
// correspond when they have corresponding signatures and equivalent
// template-heads; otherwise each declares its own function template.
#include <cassert>

static int value_calls;
static int pointer_calls;

template <class Value> int describe(Value value);

template <class Value> int describe(Value *pointer) {
  ++pointer_calls;
  return *pointer + 20;
}

int main() {
  int amount = 3;

  assert(describe(amount) == 13);
  assert(value_calls == 1);
  assert(pointer_calls == 0);

  assert(describe(&amount) == 23);
  assert(value_calls == 1);
  assert(pointer_calls == 1);
  return 0;
}

template <class Renamed> int describe(Renamed renamed) {
  ++value_calls;
  return renamed + 10;
}
