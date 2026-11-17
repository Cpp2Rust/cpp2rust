// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <utility>

// N4950 [utility.swap]/3: swap(a, b) exchanges the values stored in two
// locations.
namespace p3_swap_pointers {
void test() {
  int first = 1, second = 2;
  int *p = &first, *q = &second;
  std::swap(p, q);
  assert(p == &second && q == &first);
  assert(first == 1 && second == 2);
  int *null = nullptr;
  std::swap(null, p);
  assert(null == &second && p == nullptr);
}
} // namespace p3_swap_pointers

int main() {
  p3_swap_pointers::test();
  return 0;
}
