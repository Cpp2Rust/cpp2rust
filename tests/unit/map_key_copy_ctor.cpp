#include <cstdio>
#include <map>
#include <utility>

int copies = 0;
int moves = 0;
int made = 0;

struct Key {
  int k;
  Key(int k) : k(k) {}
  Key(const Key &o) : k(o.k) { ++copies; }
  Key(Key &&o) : k(o.k) { ++moves; }
  bool operator<(const Key &o) const { return k < o.k; }
};

Key make_key(int k) {
  ++made;
  return Key(k);
}

int main() {
  bool ok = true;
  std::map<Key, int> m;
  Key a(1), b(2), c(3);

  // A key is copied once when inserted, and not when found.
  m[a] = 10;
  ok &= (copies == 1);
  m[b] = 20;
  ok &= (copies == 2);
  m[a] = 11;
  ok &= (copies == 2);
  ok &= (m.find(a)->second == 11);
  ok &= (m.at(b) == 20);
  ok &= (copies == 2);

  // An rvalue key is moved when inserted, and the key expression is
  // evaluated once.
  m[make_key(4)] = 40;
  m[make_key(4)] = 41;
  ok &= (made == 2 && moves == 1 && copies == 2);
  m[std::move(c)] = 30;
  m[std::move(c)] = 31;
  ok &= (moves == 2 && copies == 2 && c.k == 3);
  ok &= (m.size() == 4);

  // A key in the map itself.
  m[m.begin()->first] += 1;
  ok &= (copies == 2 && m[a] == 12);

  // The key of an entry is the same object while it is in the map.
  const Key *first = &m.begin()->first;
  m[Key(0)] = 0;
  ok &= (moves == 3);
  ok &= (&m.find(a)->first == first);

  // Copying the map copies each key once.
  std::map<Key, int> n = m;
  ok &= (copies == 2 + 5);
  ok &= (n.size() == 5 && n[a] == 12);

  // Erasing copies no key.
  m.erase(m.find(a));
  ok &= (m.erase(b) == 1);
  ok &= (m.erase(b) == 0);
  ok &= (m.erase(m.begin()->first) == 1);
  ok &= (copies == 7 && moves == 3);
  ok &= (m.size() == 2 && n.size() == 5);

  int sum = 0;
  for (const auto &entry : m)
    sum += entry.first.k * entry.second;
  ok &= (sum == 3 * 31 + 4 * 41);
  ok &= (copies == 7 && moves == 3);

  printf("%d %d %d\n", copies, moves, made);
  return ok ? 0 : 1;
}
