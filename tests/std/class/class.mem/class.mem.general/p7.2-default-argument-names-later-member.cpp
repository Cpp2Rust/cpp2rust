// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mem.general]/7.2: a default argument within the
// member-specification of a class is a complete-class context.
#include <cassert>

int step = 100;

struct HasLaterStaticStep {
  int total = 1;

  int advance(int amount = step) {
    total += amount;
    return total;
  }

  static inline int step = 4;
};

int main() {
  HasLaterStaticStep object;
  assert(object.advance() == 5);
  assert(object.advance(2) == 7);
  assert(step == 100);
  return 0;
}
