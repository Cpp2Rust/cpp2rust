// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.scope.param]/1.1: the scope of a function's parameters extends
// to the end of the function-definition; a member function's parameter scope
// is nested within its class's scope.
namespace p1_1_parameter_hides_member_in_constructor {
struct HasMemberNamedLikeParameter {
  int amount;

  HasMemberNamedLikeParameter(int amount) : amount(amount + 1) {
    amount += 10;
    assert(amount == 14);
    assert(this->amount == 5);
  }

  int add(int amount) const { return this->amount + amount; }
};

void test() {
  HasMemberNamedLikeParameter object(4);
  assert(object.amount == 5);
  assert(object.add(20) == 25);
  assert(object.amount == 5);
}
} // namespace p1_1_parameter_hides_member_in_constructor

// N4950 [basic.scope.param]/1.1: the scope of a function's parameters extends
// to the end of the function-definition.
namespace p1_1_parameter_hides_namespace_variable {
int rate = 3;

int scale(int rate) {
  rate *= 2;
  return rate;
}

int read_rate() { return rate; }

void test() {
  assert(scale(5) == 10);
  assert(rate == 3);
  assert(read_rate() == 3);
}
} // namespace p1_1_parameter_hides_namespace_variable

// N4950 [basic.scope.param]/1.2: the scope of a lambda-declarator's parameters
// extends to the end of the compound-statement of the lambda-expression.
namespace p1_2_lambda_parameter_hides_enclosing_variable {
void test() {
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
}
} // namespace p1_2_lambda_parameter_hides_enclosing_variable

int main() {
  p1_1_parameter_hides_member_in_constructor::test();
  p1_1_parameter_hides_namespace_variable::test();
  p1_2_lambda_parameter_hides_enclosing_variable::test();
  return 0;
}
