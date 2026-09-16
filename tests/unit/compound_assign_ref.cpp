#include <cassert>
#include <vector>

int main() {
  std::vector<int> v;
  v.push_back(10);
  v.front() += 5;
  assert(v.front() == 15);
  return 0;
}
