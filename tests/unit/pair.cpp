#include <assert.h>
#include <utility>

int main() {
  std::pair<double, int> p1(1, 2);
  assert(p1.first == 1.0);
  assert(p1.second == 2);
  std::pair<double, int> p2{3, 4};
  assert(p2.first == 3.0);
  assert(p2.second == 4);
  std::pair<double, int> p3 = std::make_pair(5, 6);
  assert(p3.first == 5.0);
  assert(p3.second == 6);
  return 0;
}
