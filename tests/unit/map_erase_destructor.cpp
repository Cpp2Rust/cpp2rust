// XFAIL: unsafe
#include <cassert>
#include <map>
#include <utility>

int destroyed[32];

struct Tracked {
  int id;
  Tracked() : id(0) {}
  Tracked(int id) : id(id) {}
  ~Tracked() { ++destroyed[id]; }
};

bool operator<(const Tracked &a, const Tracked &b) { return a.id < b.id; }

template <typename K, typename V>
void check_erase(bool key_destructor, bool value_destructor) {
  K one(1), two(2), three(3);
  std::map<K, V> m;
  m[one] = V(11);
  m[two] = V(12);
  m[three] = V(13);
  for (int &n : destroyed)
    n = 0;

  {
    auto first = m.find(one);
    auto middle = m.find(two);
    auto last = m.find(three);
    auto retained = middle;
    typename std::map<K, V>::const_iterator retained_const = middle;
    auto *first_value = &first->second;
    auto *last_value = &last->second;
    auto end = m.end();

    auto next = m.erase(middle);
    assert(destroyed[2] == key_destructor);
    assert(destroyed[12] == value_destructor);
    assert(destroyed[1] == 0 && destroyed[3] == 0);
    assert(destroyed[11] == 0 && destroyed[13] == 0);
    assert(m.size() == 2 && m.find(two) == end);
    assert(next == last && first == m.begin());
    assert(&first->second == first_value && &last->second == last_value);
    auto walking = first;
    ++walking;
    assert(walking == last);
    ++walking;
    assert(walking == end);

    typename std::map<K, V>::const_iterator const_first = first;
    next = m.erase(const_first);
    assert(destroyed[1] == key_destructor);
    assert(destroyed[11] == value_destructor);
    assert(m.size() == 1 && next == last && m.begin() == last);
    assert(&last->second == last_value);

    next = m.erase(last);
    assert(destroyed[3] == key_destructor);
    assert(destroyed[13] == value_destructor);
    assert(next == end && m.size() == 0 && m.begin() == end);
    // The copies of erased iterators are retained, but never used.
  }
  for (int i = 0; i < 32; ++i) {
    int expected = 0;
    if (i >= 1 && i <= 3)
      expected = key_destructor;
    if (i >= 11 && i <= 13)
      expected = value_destructor;
    assert(destroyed[i] == expected);
  }
}

struct MoveOnly {
  int id;
  MoveOnly(int id) : id(id) {}
  MoveOnly(const MoveOnly &) = delete;
  MoveOnly(MoveOnly &&other) : id(other.id) {}
  ~MoveOnly() { ++destroyed[id]; }
};

bool operator<(const MoveOnly &a, const MoveOnly &b) { return a.id < b.id; }

void check_nested_move_only() {
  // The inner map is default-constructed by operator[], not a constructor rule.
  std::map<int, std::map<MoveOnly, Tracked>> outer;
  auto &m = outer[0];
  MoveOnly key(4);
  m[std::move(key)] = Tracked(14);
  destroyed[4] = destroyed[14] = 0;
  auto next = m.erase(m.begin());
  assert(destroyed[4] == 1 && destroyed[14] == 1);
  assert(m.size() == 0 && next == m.end());
}

int main() {
  check_erase<Tracked, Tracked>(true, true);
  check_erase<Tracked, int>(true, false);
  check_erase<int, Tracked>(false, true);
  check_erase<int, int>(false, false);
  check_nested_move_only();
}
