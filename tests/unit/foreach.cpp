#include <cassert>
#include <vector>

int main() {
  std::vector<int> v;
  for (int i = 0; i < 10; ++i)
    v.push_back(i);
  int sum = 0;
  for (int x : v)
    sum += x;
  assert(sum == 45);
  return 0;
}
