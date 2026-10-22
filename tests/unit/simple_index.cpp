#include <cassert>
#include <vector>

int main() {
  std::vector<bool> v = {true};
  assert(v[0]);
  std::vector<bool>::reference r = v[0];
  std::vector<bool>::reference copy = r;
  assert(copy);
  return 0;
}
