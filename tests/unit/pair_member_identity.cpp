#include <cassert>
#include <utility>

int main() {
  std::pair<int, int> p{1, 2};
  std::pair<int, int> q{3, 4};
  int *f = &p.first;
  p = q;
  assert(*f == 3);
  std::pair<int, int> &alias = p;
  p = alias;
  assert(*f == 3);

  std::pair<int, int> a{1, 2};
  int *pa = &a.first;
  std::pair<int, int> b = std::move(a);
  *pa = 9;
  assert(b.first == 1);
  assert(a.first == 9);

  std::pair<int, int> d{1, 2};
  std::pair<int, int> s{3, 4};
  int *pd = &d.first;
  int *ps = &s.first;
  d = std::move(s);
  assert(*pd == 3);
  *ps = 5;
  assert(d.first == 3);
  assert(s.first == 5);

  std::pair<long, long> w{1, 2};
  std::pair<int, int> n{3, 4};
  long *pw = &w.first;
  w = n;
  assert(*pw == 3);
  std::pair<int, int> m{5, 6};
  w = std::move(m);
  assert(*pw == 5);
  return 0;
}
