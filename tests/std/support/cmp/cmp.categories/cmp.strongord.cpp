#include <cassert>
#include <compare>

// N4950 [cmp.strongord]/4: Returns: v.value @ 0 for operator@.
namespace p4_compare_with_literal_zero {
int less_than_zero(std::strong_ordering o) { return o < 0; }

void test() {
  std::strong_ordering o = 1 <=> 2;
  int a = o < 0;
  assert(a == 1);
  assert(less_than_zero(o) == 1);
  assert(static_cast<int>(o < 0) == 1);
  long d = o > 0;
  assert(d == 0);
}
} // namespace p4_compare_with_literal_zero

int main() {
  p4_compare_with_literal_zero::test();
  return 0;
}
