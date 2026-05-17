#include <cassert>
#include <cstddef>
#include <initializer_list>
#include <vector>

size_t f(std::initializer_list<int> bytes) {
  std::vector<int> *buf = new std::vector<int>(bytes);
  size_t n = bytes.size();
  delete buf;
  return n;
}

int main() {
  assert(f({1, 2, 3}) == 3);
  return 0;
}
