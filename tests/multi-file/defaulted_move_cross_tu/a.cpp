#include <assert.h>

#include "s.h"

int sum(const S &s) { return static_cast<int>(s.v.size()) + s.n[0] + s.n[1]; }

int main() {
  S s(2);
  assert(sum(s) == 7);
  assert(shuffle(3) == 10);
  T t{{1, 2}};
  assert(t.size() == 2);
  return 0;
}
