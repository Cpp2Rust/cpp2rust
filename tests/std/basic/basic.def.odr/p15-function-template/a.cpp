#include "h.h"
#include <cassert>

AssertsDtorCountAtExit check{2};

int main() {
  assert(LifetimeTracked::ctorCount == 0);
  assert(count<int>() == 1);
  assert(b_count_int() == 2);
  assert(count<int>() == 3);
  assert(LifetimeTracked::ctorCount == 1);
  assert(count<long>() == 1);
  assert(b_count_long() == 2);
  assert(LifetimeTracked::ctorCount == 2);
  return 0;
}
