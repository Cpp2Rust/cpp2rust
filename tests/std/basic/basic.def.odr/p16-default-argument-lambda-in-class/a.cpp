#include "h.h"
#include <cassert>

AssertsDtorCountAtExit check{1};

int main() {
  assert(LifetimeTracked::ctorCount == 0);
  X x;
  x.h(true);
  assert(LifetimeTracked::ctorCount == 1);
  assert(LifetimeTracked::useCount == 1);
  b_call();
  assert(LifetimeTracked::ctorCount == 1);
  assert(LifetimeTracked::useCount == 2);
  return 0;
}
