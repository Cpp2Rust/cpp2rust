#include <algorithm>
#include <cassert>
#include <vector>

struct Degrees {
  int value;
  Degrees(int v) : value(v) {}
};

int main() {
  int input[] = {1, 2, 3};
  int output[3];
  std::copy(input, input + 3, output);

  std::vector<long> widened(3, 0L);
  std::copy(input, input + 3, widened.begin());
  assert(widened[0] == 1L && widened[2] == 3L);

  std::vector<Degrees> constructed(3, Degrees(0));
  std::copy(input, input + 3, constructed.begin());
  assert(constructed[0].value == 1 && constructed[2].value == 3);

  long wide[2] = {7L, 8L};
  std::vector<double> floating(2, 0.0);
  std::copy(wide, wide + 2, floating.begin());
  assert(floating[1] == 8.0);
  return 0;
}
