#include <vector>

void foo(std::vector<int>::iterator a0) {}

int main() {
  std::vector<int> vec(4);
  std::vector<int>::iterator it = vec.begin();
  foo(it);
  return 0;
}
