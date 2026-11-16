// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/2.11: the name of an unscoped enumerator is bound
// in the enumeration scope and in the scope enclosing the enumeration.
#include <cassert>

enum Direction { north = 4, south = 9 };

struct HasUnscopedEnum {
  enum Size { small = 3, large = 8 };

  int scale(int count) const { return count * large; }
};

int main() {
  assert(north == 4);
  assert(Direction::north == 4);
  assert(south == 9);
  assert(Direction::south == 9);

  assert(HasUnscopedEnum::small == 3);
  assert(HasUnscopedEnum::Size::small == 3);
  assert(HasUnscopedEnum::large == 8);
  assert(HasUnscopedEnum::Size::large == 8);

  HasUnscopedEnum object;
  assert(object.scale(2) == 16);
  assert(object.small == 3);
  return 0;
}
