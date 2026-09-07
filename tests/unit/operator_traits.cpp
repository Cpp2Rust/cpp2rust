// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <algorithm>
#include <cassert>
#include <compare>
#include <map>
#include <vector>

struct Lt {
  int v;
  bool operator<(const Lt &o) const { return v < o.v; }
};

struct Eq {
  int v;
  bool operator==(const Eq &o) const { return v == o.v; }
};

struct Cmp {
  int v;
  std::strong_ordering operator<=>(const Cmp &o) const { return v <=> o.v; }
  bool operator==(const Cmp &o) const { return v == o.v; }
};

struct Free {
  int v;
};
bool operator<(const Free &a, const Free &b) { return a.v < b.v; }
bool operator==(const Free &a, const Free &b) { return a.v == b.v; }

int main() {
  std::vector<Lt> lts{{3}, {1}, {2}};
  std::sort(lts.begin(), lts.end());
  assert(lts[0].v == 1 && lts[1].v == 2 && lts[2].v == 3);

  std::vector<Eq> eqs{{1}, {2}, {3}};
  Eq two{2}, nine{9};
  assert(std::find(eqs.begin(), eqs.end(), two) - eqs.begin() == 1);
  assert(std::find(eqs.begin(), eqs.end(), nine) == eqs.end());

  std::vector<Cmp> cmps{{3}, {1}, {2}};
  std::sort(cmps.begin(), cmps.end());
  assert(cmps[0].v == 1 && cmps[2].v == 3);
  Cmp three{3};
  assert(std::find(cmps.begin(), cmps.end(), three) - cmps.begin() == 2);

  std::vector<Free> frees{{2}, {1}};
  std::sort(frees.begin(), frees.end());
  assert(frees[0].v == 1);
  Free ftwo{2};
  assert(std::find(frees.begin(), frees.end(), ftwo) - frees.begin() == 1);

  std::map<Lt, int> m;
  m[Lt{2}] = 20;
  m[Lt{1}] = 10;
  assert(m.begin()->second == 10);
  assert(m[Lt{2}] == 20);
  return 0;
}
