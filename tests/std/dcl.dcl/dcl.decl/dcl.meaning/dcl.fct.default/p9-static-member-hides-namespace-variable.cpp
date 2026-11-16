// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.fct.default]/9: a non-static member shall not appear in a default
// argument; a static member needs no object and can be named there.
#include <cassert>

int step = 100;

struct HasStaticStep {
  static inline int step = 4;

  int total = 1;

  int advance(int amount = step) {
    total += amount;
    return total;
  }
};

int main() {
  HasStaticStep object;
  assert(object.advance() == 5);

  HasStaticStep::step = 6;
  assert(object.advance() == 11);
  assert(object.advance(2) == 13);
  assert(step == 100);
  return 0;
}
