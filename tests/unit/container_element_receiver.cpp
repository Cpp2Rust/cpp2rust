#include <assert.h>
#include <map>
#include <string.h>
#include <string>
#include <vector>

struct Holder {
  std::vector<int> &ref;
  std::vector<int> vec;
};

std::vector<int> &pick(std::vector<int> &a, std::vector<int> &b, bool first) {
  return first ? a : b;
}

int sum(const std::vector<int> &v) {
  int total = 0;
  for (auto it = v.begin(); it != v.end(); ++it) {
    total += *it;
  }
  return total + v.at(0);
}

int main() {
  std::vector<int> v = {1, 2, 3};
  Holder h{v, {4, 5}};

  // Through a reference field, a field, and a pointer.
  assert(*h.ref.data() == 1);
  h.ref.front() = 9;
  assert(v[0] == 9);
  h.ref.front() = 1;
  assert(h.vec.back() == 5);
  std::vector<int> *p = &h.vec;
  assert(*p->data() == 4);

  // Through a call that returns a reference, and a map element.
  pick(v, h.vec, false).front() = 7;
  assert(h.vec[0] == 7);
  std::map<int, std::vector<int>> m;
  m[1].push_back(8);
  assert(*m[1].begin() == 8);

  // Const vector::at.
  const std::vector<int> &c = v;
  assert(c.at(2) == 3);
  assert(sum(v) == 7);
  return 0;
}
