// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.scope.enum]/1: any declaration of an enumeration E introduces
// an enumeration scope that includes the enumerator-list of the enum-specifier
// for E.
namespace p1_enumerator_initializer_finds_earlier_enumerator {
const int first = 100;

enum class Step { first = 2, second = first * 5 };
enum class Stride { single = first + 1 };

void test() {
  assert(static_cast<int>(Step::first) == 2);
  assert(static_cast<int>(Step::second) == 10);
  assert(static_cast<int>(Stride::single) == 101);
  assert(first == 100);
}
} // namespace p1_enumerator_initializer_finds_earlier_enumerator

// N4950 [basic.scope.enum]/1: any declaration of an enumeration E introduces
// an enumeration scope that includes the enumerator-list of the enum-specifier
// for E.
namespace p1_scoped_enumerator_does_not_hide_outer_name {
enum class Power { off = 3, on = 8 };

int off = 40;

void test() {
  assert(off == 40);
  assert(static_cast<int>(Power::off) == 3);

  off += static_cast<int>(Power::on);
  assert(off == 48);
  assert(static_cast<int>(Power::off) == 3);
}
} // namespace p1_scoped_enumerator_does_not_hide_outer_name

int main() {
  p1_enumerator_initializer_finds_earlier_enumerator::test();
  p1_scoped_enumerator_does_not_hide_outer_name::test();
  return 0;
}
