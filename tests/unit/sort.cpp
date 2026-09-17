#include <algorithm>
#include <assert.h>
#include <vector>

int main() {
  std::vector<int> v;
  v.push_back(10);
  v.push_back(1);
  v.push_back(9);
  v.push_back(2);
  v.push_back(8);
  v.push_back(3);
  v.push_back(7);
  v.push_back(4);
  v.push_back(5);
  v.push_back(6);
  std::sort(v.begin(), v.end());
  for (unsigned i = 0; i < v.size() - 1; ++i)
    assert(v[i] < v[i + 1]);
  return 0;
}
