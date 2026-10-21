#include <assert.h>
#include <vector>

int main() {
  std::vector<int> v = {1, 2, 3, 4};
  auto it = v.begin();
  ++it;
  auto r = v.erase(it++);
  assert(*r == 3);
  assert(v.size() == 3);
  auto it2 = v.begin();
  r = v.insert(it2++, 9);
  assert(*r == 9 && v[0] == 9 && v.size() == 4);
  const std::vector<int> &c = v;
  int i = 1;
  assert(c.at(i++) == 1 && i == 2);
  return 0;
}
