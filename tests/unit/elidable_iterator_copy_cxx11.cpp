// ADDITIONAL_COMPILE_FLAGS: -std=c++11
#include <algorithm>
#include <cassert>
#include <vector>

int main() {
  std::vector<int> v;
  v.push_back(4);
  v.push_back(7);
  std::vector<int>::iterator it = std::find(v.begin(), v.end(), 7);
  assert(it != v.end());
  assert(*it == 7);
  std::random_shuffle(v.begin(), v.end());
  std::sort(v.begin(), v.end());
  assert(v[0] == 4 && v[1] == 7);
  return 0;
}
