#include <assert.h>
#include <utility>
#include <vector>

int main() {
  std::pair<double, int> p1(1, 2);
  assert(p1.first == 1.0);
  assert(p1.second == 2);
  std::pair<double, int> p2{3, 4};
  assert(p2.first == 3.0);
  assert(p2.second == 4);
  std::pair<double, int> p3 = std::make_pair(5, 6);
  assert(p3.first == 5.0);
  assert(p3.second == 6);
  const int arr[2] = {1, 2};
  unsigned n = 2;
  std::vector<std::pair<const int *, size_t>> const_ptr_pairs(1);
  const_ptr_pairs[0] = std::make_pair(arr + 1, n);
  assert(*const_ptr_pairs[0].first == 2 && const_ptr_pairs[0].second == 2);

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
  std::pair<int, int> narrow{3, 4};
  long *pw = &w.first;
  w = narrow;
  assert(*pw == 3);
  std::pair<int, int> m{5, 6};
  w = std::move(m);
  assert(*pw == 5);
  return 0;
}
