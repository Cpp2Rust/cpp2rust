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
  std::vector<int> v = {4, 5, 6};
  assert(v.size() == 3);
  assert(v[0] + v[1] + v[2] == 15);
  std::initializer_list<int> l = {7, 8};
  assert(l.size() == 2);
  return 0;
}
