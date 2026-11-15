#pragma once

#include "lifetime-tracked.h"
#include <cassert>

struct AssertsDtorCountAtExit {
  int expected;
  ~AssertsDtorCountAtExit() { assert(LifetimeTracked::dtorCount == expected); }
};
