#include <assert.h>

#include "s.h"

int main() {
  S<int> p;
  p.set(3);
  assert(f(&p) == 3);
  return 0;
}
