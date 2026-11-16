// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.param]/1.2: the scope of a lambda-declarator's parameters
// extends to the end of the compound-statement of the lambda-expression.
#include <cassert>

int main() {
  int amount = 6;
  int total = 0;
  auto add_doubled = [&](int amount) {
    amount *= 2;
    total += amount;
    return amount;
  };
  assert(add_doubled(4) == 8);
  assert(total == 8);
  assert(amount == 6);
  return 0;
}
