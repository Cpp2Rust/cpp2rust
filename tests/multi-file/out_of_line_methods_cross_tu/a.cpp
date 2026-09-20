#include <assert.h>

#include "s.h"

int main() {
  S s(1);
  assert(s.get() == 1);
  s.set(4);
  assert(s.get() == 4);
  assert(s.add(2) == 6);
  return 0;
}
