#include <array>
#include <initializer_list>
#include <vector>

void f(std::initializer_list<int> list) {}

int main() {
  int i1{3};
  int i2{};
  int carr1[] = {1, 2};
  int carr2[3] = {1};
  std::array<int, 3> arr = {1, 2, 3};
  std::vector<int> vec = {1, 2, 3};
  f({1, 2, 3, 4});

  return 0;
}
