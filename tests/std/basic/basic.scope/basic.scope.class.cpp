// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.scope.class]/1: any declaration of a class C introduces a class
// scope that includes the member-specification of the class-specifier for C.
namespace p1_member_hides_namespace_variable {
int balance = 50;

struct HasBalance {
  int balance = 7;

  int deposit(int amount) {
    balance += amount;
    return balance;
  }
  int deposit_globally(int amount) {
    p1_member_hides_namespace_variable::balance += amount;
    return p1_member_hides_namespace_variable::balance;
  }
};

void test() {
  HasBalance object;
  assert(object.deposit(3) == 10);
  assert(object.balance == 10);
  assert(balance == 50);

  assert(object.deposit_globally(4) == 54);
  assert(object.balance == 10);
  assert(balance == 54);
}
} // namespace p1_member_hides_namespace_variable

// N4950 [basic.scope.class]/1: for each non-friend redeclaration whose target
// scope is the class scope, the portion after the declarator-id is also
// included in the scope.
namespace p1_out_of_class_definition_is_in_class_scope {
int initial = 100;

struct HasStaticTotal {
  typedef int Amount;

  static int initial;
  static int total;

  int held = 1;

  Amount add(Amount amount);
};

int HasStaticTotal::initial = 6;
int HasStaticTotal::total = initial + 1;

HasStaticTotal::Amount HasStaticTotal::add(Amount amount) {
  held += amount;
  total += initial;
  return held;
}

void test() {
  assert(HasStaticTotal::total == 7);

  HasStaticTotal object;
  assert(object.add(4) == 5);
  assert(HasStaticTotal::total == 13);
  assert(HasStaticTotal::initial == 6);
  assert(initial == 100);
}
} // namespace p1_out_of_class_definition_is_in_class_scope

int main() {
  p1_member_hides_namespace_variable::test();
  p1_out_of_class_definition_is_in_class_scope::test();
  return 0;
}
