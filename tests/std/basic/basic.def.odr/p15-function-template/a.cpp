#include <cassert>

#include "../../../support/asserts-dtor-count-at-exit.h"
#include "h.h"

AssertsDtorCountAtExit check{4};

int main() {
  assert(LifetimeTracked::ctorCount == 0);
  struct S {};
  assert(count<S>() == 1);
  assert(b_count_local() == 1);
  assert(count<S>() == 2);
  assert(count<int>() == 1);
  assert(b_count_int() == 2);
  assert(count<int>() == 3);
  assert(LifetimeTracked::ctorCount == 3);
  assert(count<long>() == 1);
  assert(b_count_long() == 2);
  assert(LifetimeTracked::ctorCount == 4);
  return 0;
}
