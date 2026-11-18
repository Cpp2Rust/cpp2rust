#include <cassert>

#include "h.h"

int main() {
  HasInlineMembers::shared.use();
  HasInlineMembers::instance().use();
  b_use();
  assert(LifetimeTracked::ctorCount == 2);
  assert(HasInlineMembers::shared.uses == 2);
  assert(HasInlineMembers::instance().uses == 2);
  assert(&HasInlineMembers::shared == b_shared_address());
  assert(&HasInlineMembers::instance() == b_instance_address());
  assert(LifetimeTracked::copyCount == 0);
  return 0;
}
