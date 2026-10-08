#include <assert.h>

#include "s.h"

int main() {
  S s(1);
  assert(s.get() == 1);
  s.set(4);
  assert(s.get() == 4);
  assert(s.add(2) == 6);

  Derived derived(3);
  Base *base = &derived;
  assert(base->apply(5) == 15);

  Pair pair{7, 3};
  assert(pair_sum(&pair) == 10);
  assert(pair_diff(&pair) == 4);
  return 0;
}
