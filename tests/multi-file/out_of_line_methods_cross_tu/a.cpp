#include <assert.h>

#include "s.h"

int main() {
  S s(1);
  assert(s.get() == 1);
  s.set(4);
  assert(s.get() == 4);
  assert(s.add(2) == 6);

  D d(3);
  Base *b = &d;
  assert(b->scale(5) == 15);
  return 0;
}
