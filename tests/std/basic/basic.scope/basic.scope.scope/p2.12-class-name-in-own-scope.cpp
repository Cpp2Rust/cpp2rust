// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/2.12: a class's name is also bound in its own
// scope.
#include <cassert>

struct HasStaticCounter {
  static inline int count = 0;

  static void bump() { ++HasStaticCounter::count; }
};

int main() {
  HasStaticCounter::bump();
  assert(HasStaticCounter::count == 1);

  HasStaticCounter::HasStaticCounter::bump();
  assert(HasStaticCounter::count == 2);

  HasStaticCounter::HasStaticCounter::HasStaticCounter::count = 6;
  assert(HasStaticCounter::count == 6);
  return 0;
}
