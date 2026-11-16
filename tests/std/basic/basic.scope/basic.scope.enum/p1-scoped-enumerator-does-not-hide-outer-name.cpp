// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.enum]/1: any declaration of an enumeration E introduces
// an enumeration scope that includes the enumerator-list of the enum-specifier
// for E.
#include <cassert>

enum class Power { off = 3, on = 8 };

int off = 40;

int main() {
  assert(off == 40);
  assert(static_cast<int>(Power::off) == 3);

  off += static_cast<int>(Power::on);
  assert(off == 48);
  assert(static_cast<int>(Power::off) == 3);
  return 0;
}
