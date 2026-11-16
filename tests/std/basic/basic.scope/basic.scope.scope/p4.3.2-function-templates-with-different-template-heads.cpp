// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/4.3.2: two function template declarations
// correspond when they have corresponding signatures and equivalent
// template-heads; otherwise each declares its own function template.
#include <cassert>

static int type_parameter_calls;
static int value_parameter_calls;

template <class Value> int describe(Value value) {
  ++type_parameter_calls;
  return value + 10;
}

template <int amount> int describe() {
  ++value_parameter_calls;
  return amount + 20;
}

int main() {
  assert(describe(3) == 13);
  assert(type_parameter_calls == 1);
  assert(value_parameter_calls == 0);

  assert(describe<3>() == 23);
  assert(type_parameter_calls == 1);
  assert(value_parameter_calls == 1);
  return 0;
}
