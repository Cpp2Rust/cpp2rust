#include <cassert>
#include <utility>

int main() {
  std::pair<double, int> p(1.5, 2);
  std::tuple_element<0, std::pair<double, int>>::type first = std::get<0>(p);
  std::tuple_element<1, std::pair<double, int>>::type second = std::get<1>(p);
  std::tuple_element<1, std::pair<double, int>>::type &ref = std::get<1>(p);
  ref = 7;
  assert(first == 1.5);
  assert(second == 2);
  assert(p.second == 7);
  return 0;
}
