// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.class]/1: any declaration of a class C introduces a class
// scope that includes the member-specification of the class-specifier for C.
#include <cassert>

int balance = 50;

struct HasBalance {
  int balance = 7;

  int deposit(int amount) {
    balance += amount;
    return balance;
  }
  int deposit_globally(int amount) {
    ::balance += amount;
    return ::balance;
  }
};

int main() {
  HasBalance object;
  assert(object.deposit(3) == 10);
  assert(object.balance == 10);
  assert(balance == 50);

  assert(object.deposit_globally(4) == 54);
  assert(object.balance == 10);
  assert(balance == 54);
  return 0;
}
