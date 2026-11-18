// panic-ub
#include <vector>

int main() {
  std::vector<int> v = {1, 2, 3};
  auto idx = v.size();
  int *p = &v.at(idx);
  return p == nullptr;
}
