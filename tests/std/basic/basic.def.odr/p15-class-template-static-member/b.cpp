#include "h.h"

void b_use_int() { HasStaticMember<int>::member.use(); }

void b_use_long() { HasStaticMember<long>::member.use(); }

LifetimeTracked *b_int_address() { return &HasStaticMember<int>::member; }
