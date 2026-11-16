// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.enum]/1: any declaration of an enumeration E introduces
// an enumeration scope that includes the enumerator-list of the enum-specifier
// for E.
#include <cassert>

const int first = 100;

enum class Step { first = 2, second = first * 5 };
enum class Stride { single = first + 1 };

int main() {
  assert(static_cast<int>(Step::first) == 2);
  assert(static_cast<int>(Step::second) == 10);
  assert(static_cast<int>(Stride::single) == 101);
  assert(first == 100);
  return 0;
}
