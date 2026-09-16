#include <algorithm>
#include <vector>

template <class RandomIt> void f1(RandomIt first, RandomIt last) {
  return std::sort(first, last);
}

int main() {
  std::vector<int> v = {1, 3, 0, 2, 8, 7};
  f1(v.begin(), v.end());

  return 0;
}
