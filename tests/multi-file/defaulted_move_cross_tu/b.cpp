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
  T t{{1}};
  T u(std::move(t));
  T w;
  w = std::move(u);
  assert(t.size() == 0 && u.size() == 0 && w.size() == 1);
  return sum(c);
}
