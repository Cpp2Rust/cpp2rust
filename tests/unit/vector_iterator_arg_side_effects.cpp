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
  std::vector<int> w = {10, 20, 30, 40};
  auto a = w.end();
  auto &ra = (a -= 3);
  assert(&ra == &a && *a == 20);
  auto &rb = ++a;
  assert(&rb == &a && *a == 30);
  auto &rc = --a;
  assert(&rc == &a && *a == 20);
  auto &rd = (a += 2);
  assert(&rd == &a && *a == 40);
  auto b = w.begin();
  int x = *++b;
  assert(x == 20);
  auto k = ++b;
  ++k;
  assert(*k == 40 && *b == 30);
  return 0;
}
