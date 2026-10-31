#include <cassert>
#include <vector>

int main() {
  std::vector<bool> v1 = {true};
  std::vector<bool>::reference r1 = v1[0];
  assert(v1[0] && r1);
  r1 = false;
  assert(!v1[0]);
  auto it = v1.begin();
  auto r2 = *it;
  r2 = true;
  assert(v1[0]);
  v1[0] = false;
  assert(!v1[0]);
  return 0;
}
