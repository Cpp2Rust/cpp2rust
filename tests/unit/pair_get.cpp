#include <cassert>
#include <utility>

static int second_of(const std::pair<double, int> &p) { return std::get<1>(p); }

static double first_of(const std::pair<double, int> &p) {
  return std::get<0>(p);
}

int main() {
  std::pair<double, int> p(1.5, 2);
  assert(std::get<0>(p) == 1.5);
  assert(std::get<1>(p) == 2);
  assert(first_of(p) == 1.5);
  assert(second_of(p) == 2);
  return 0;
}
