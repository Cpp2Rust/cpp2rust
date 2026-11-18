#include "h.h"

void b_use() { shared.use(); }

LifetimeTracked *b_address() { return &shared; }
