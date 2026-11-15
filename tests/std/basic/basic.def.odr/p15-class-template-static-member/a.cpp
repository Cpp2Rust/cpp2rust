#include "h.h"
#include <cassert>

int main() {
  HasStaticMember<int>::member.use();
  b_use_int();
  b_use_long();
  assert(HasStaticMember<int>::member.uses == 2);
  assert(HasStaticMember<long>::member.uses == 1);
  assert(&HasStaticMember<int>::member == b_int_address());
  assert(LifetimeTracked::ctorCount == 2);
  assert(LifetimeTracked::copyCount == 0);
  return 0;
}
