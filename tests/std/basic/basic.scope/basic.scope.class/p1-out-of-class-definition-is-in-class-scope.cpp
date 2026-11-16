// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.class]/1: for each non-friend redeclaration whose target
// scope is the class scope, the portion after the declarator-id is also
// included in the scope.
#include <cassert>

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

int main() {
  assert(HasStaticTotal::total == 7);

  HasStaticTotal object;
  assert(object.add(4) == 5);
  assert(HasStaticTotal::total == 13);
  assert(HasStaticTotal::initial == 6);
  assert(initial == 100);
  return 0;
}
