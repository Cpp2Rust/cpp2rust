#include <cassert>
#include <utility>
#include <vector>

void foo(std::vector<int>::iterator a0) {}

int deref(std::vector<int>::iterator a0) { return *a0; }

int main() {
  std::vector<int> vec(4);
  std::vector<int>::iterator it = vec.begin();
  foo(it);
  vec[0] = 7;
  assert(deref(std::move(it)) == 7);
  assert(*it == 7);
  std::vector<int>::iterator moved = std::move(it);
  assert(*moved == 7);
  assert(*it == 7);
  return 0;
}
