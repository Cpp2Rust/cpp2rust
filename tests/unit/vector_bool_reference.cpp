#include <cassert>
#include <vector>

int main() {
  std::vector<bool> v1 = {true};
  std::vector<bool>::reference r1 = v1[0];
  assert(v1[0] && r1);
  r1 = false;
  assert(!v1[0]);
  return 0;
}
