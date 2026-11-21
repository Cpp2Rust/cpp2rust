// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <cassert>
#include <compare>

// N4950 [cmp.strongord]: "static const strong_ordering less;
// static const strong_ordering equal; static const strong_ordering equivalent;
// static const strong_ordering greater;" A reference bound to a valid value
// refers to that static data member.
namespace valid_values {
void test() {
  const std::strong_ordering &l = std::strong_ordering::less;
  const std::strong_ordering &e = std::strong_ordering::equal;
  const std::strong_ordering &g = std::strong_ordering::greater;
  assert(&l == &std::strong_ordering::less);
  assert(&e == &std::strong_ordering::equal);
  assert(&g == &std::strong_ordering::greater);
  assert(l < 0);
  assert(e == 0);
  assert(g > 0);
}
} // namespace valid_values

int main() {
  valid_values::test();
  return 0;
}
