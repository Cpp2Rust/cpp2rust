#include "h.h"

void b_use() { instance().use(); }

LifetimeTracked *b_address() { return &instance(); }
