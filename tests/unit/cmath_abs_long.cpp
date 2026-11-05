#include <cassert>
#include <cmath>

int main() {
  long l = -5000000000L;
  long long ll = -6000000000LL;
  assert(std::abs(l) == 5000000000L);
  assert(std::abs(ll) == 6000000000LL);
  long small = -7;
  long long small_ll = -8;
  int narrowed = std::abs(small);
  int narrowed_ll = std::abs(small_ll);
  assert(narrowed == 7);
  assert(narrowed_ll == 8);
  return 0;
}
