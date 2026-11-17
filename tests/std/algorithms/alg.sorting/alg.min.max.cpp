// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <algorithm>
#include <cassert>

// N4950 [alg.min.max]/2: min returns the smaller value.
namespace p2_min_pointers {
void test() {
  int values[2] = {2, 1};
  int *low = &values[0];
  int *high = &values[1];
  assert(std::min(low, high) == low);
  assert(std::min(high, low) == low);
}
} // namespace p2_min_pointers

// N4950 [alg.min.max]/10: max returns the larger value.
namespace p10_max_pointers {
void test() {
  int values[2] = {2, 1};
  int *low = &values[0];
  int *high = &values[1];
  assert(std::max(low, high) == high);
  assert(std::max(high, low) == high);
}
} // namespace p10_max_pointers

int main() {
  p2_min_pointers::test();
  p10_max_pointers::test();
  return 0;
}
