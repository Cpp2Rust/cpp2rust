#include <assert.h>
#include <map>
#include <vector>

void foo(unsigned x) { x = x + 1; }
void bar(unsigned &x) { x = x + 1; }

int main() {
  std::map<short, unsigned> m;
  m[0] = 1;
  m[1] = 2;
  m[2] = 3;
  assert(m.size() == 3);
  assert(m[0] == 1);
  assert(m[1] == 2);
  assert(m[2] == 3);
  int x = 4;
  m[1] = x;
  assert(m.size() == 3);
  assert(m[0] == 1);
  assert(m[1] == 4);
  assert(m[2] == 3);
  foo(m[0]);
  assert(m[0] == 1);
  bar(m[2]);
  assert(m[2] == 4);
  m[0] = m[0] + m[2];
  assert(m[0] == 5);
  std::map<short, unsigned>::iterator end = m.end();
  std::map<short, unsigned>::iterator it = m.find(1);
  std::map<short, unsigned>::const_iterator const_it = m.find(10);
  unsigned x1 = it == end ? 0 : it->second;
  assert(x1 == 4);
  unsigned x2 = const_it == end ? 0 : const_it->second;
  assert(x2 == 0);
  unsigned x3 = it == m.end() ? 0 : it->second;
  assert(x3 == 4);
  unsigned x4 = const_it == m.end() ? 0 : const_it->second;
  assert(x4 == 0);
  m[4] = 5;
  std::map<short, unsigned>::iterator it4 = m.find(4);
  unsigned *p = &it4->second;
  unsigned x5 = *p;
  assert(m[4] == 5);
  assert(it4->second == 5);
  assert(*p == 5);
  assert(x5 == 5);
  ++*p;
  assert(m[4] == 6);
  assert(it4->second == 6);
  assert(*p == 6);
  assert(x5 == 5);
  std::map<short, unsigned>::const_iterator const_it4 = m.find(4);
  assert((*const_it4).second == 6);
  auto &r = m;
  assert(r.size() == 4);
  assert(m.find(4) != m.end());
  r.erase(it4);
  assert(r.size() == 3);
  assert(m.find(4) == m.end());
  m[7] = 7;
  m[8] = 8;
  m[9] = 9;
  assert(r.size() == 6);
  assert(m.erase(10) == 0);
  int key9 = 9;
  assert(m.erase(key9) == 1);
  assert(r.size() == 5);
  std::map<short, unsigned>::const_iterator cit = m.find(7);
  std::map<short, unsigned>::iterator next = m.erase(cit);
  assert(next == m.find(8));
  next = m.erase(next);
  assert(next == m.end());
  assert(r.size() == 3);
  assert(m.find(8) == m.end());
  std::map<std::pair<int, long>, double> other_map;
  assert(other_map.size() == 0);
  std::pair<int, long> key0(1, 1);
  double value = 2;
  other_map[key0] = value;
  value = other_map[key0];
  assert(other_map.size() == 1);
  assert(other_map[key0] == value);
  assert(m.size() == 3);
  int k = 0;
  assert(m.at(k) == 5);
  ++k;
  assert(m.at(k) == 4);
  ++k;
  assert(m.at(k) == 4);
  std::map<int, bool> m2;
  assert(m2.size() == 0);
  std::vector<int> indexes;
  for (unsigned i = 60; i > 30; --i)
    indexes.push_back(i);
  for (unsigned i = 100; i > 60; --i)
    indexes.push_back(i);
  for (unsigned i = 30; i > 0; --i)
    indexes.push_back(i);
  for (unsigned i = 0; i < indexes.size(); ++i)
    m2[indexes[i]] = i % 2;
  assert(m2.size() == indexes.size());
  int last = -1;
  for (auto &pair : m2) {
    assert(pair.first > last);
    assert(pair.second == pair.first % 2);
    last = pair.first;
  }
  std::map<int, int> m3;
  auto inserted = m3.emplace(1, 2);
  assert(inserted.second);
  assert(inserted.first->second == 2);
  auto not_inserted = m3.emplace(1, 3);
  assert(!not_inserted.second);
  assert(not_inserted.first->second == 2);
  m3.emplace(std::make_pair(4, 5));
  assert(m3.size() == 2);
  assert(m3[1] == 2);
  assert(m3[4] == 5);
  k = 0;
  const auto &value_0 = m.at(k);
  assert(m.size() + x1 + x2 + x3 + x4 + x5 + value_0 == 21);
  return 0;
}
