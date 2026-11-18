#include "h.h"

void b_use() {
  HasInlineMembers::shared.use();
  HasInlineMembers::instance().use();
}

LifetimeTracked *b_shared_address() { return &HasInlineMembers::shared; }

LifetimeTracked *b_instance_address() { return &HasInlineMembers::instance(); }
