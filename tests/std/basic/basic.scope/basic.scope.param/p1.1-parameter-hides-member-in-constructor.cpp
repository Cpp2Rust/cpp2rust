// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.param]/1.1: the scope of a function's parameters extends
// to the end of the function-definition; a member function's parameter scope
// is nested within its class's scope.
#include <cassert>

struct HasMemberNamedLikeParameter {
  int amount;

  HasMemberNamedLikeParameter(int amount) : amount(amount + 1) {
    amount += 10;
    assert(amount == 14);
    assert(this->amount == 5);
  }

  int add(int amount) const { return this->amount + amount; }
};

int main() {
  HasMemberNamedLikeParameter object(4);
  assert(object.amount == 5);
  assert(object.add(20) == 25);
  assert(object.amount == 5);
  return 0;
}
