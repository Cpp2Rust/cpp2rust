#include <assert.h>
#include <utility>

#include "s.h"

int shuffle(int x) {
  S a(x);
  S b(std::move(a));
  assert(a.v.empty());
  S c(1);
  c = std::move(b);
  assert(b.v.empty());
  return sum(c);
}
