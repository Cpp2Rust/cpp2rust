#include <algorithm>
#include <cassert>

struct X {
  int v;
};

int operator==(X a, X b) { return a.v == b.v ? 2 : 0; }
int operator!=(X a, X b) { return a.v != b.v ? 3 : 0; }
int operator<(X a, X b) { return a.v < b.v ? 4 : 0; }
int operator>(X a, X b) { return a.v > b.v ? 5 : 0; }
int operator<=(X a, X b) { return a.v <= b.v ? 6 : 0; }
int operator>=(X a, X b) { return a.v >= b.v ? 7 : 0; }

struct Result {
  int r;
};

struct Custom {
  int v;
  Result operator<=>(const Custom &o) const { return Result{v - o.v}; }
};

struct Mixed {
  int v;
  Result operator<=>(const Mixed &o) const { return Result{v - o.v}; }
  int operator<(const Mixed &o) const { return v < o.v ? 8 : 0; }
  int operator==(const Mixed &o) const { return v == o.v ? 9 : 0; }
};

int main() {
  X a{1}, b{2}, c{1};
  assert((a == c) == 2);
  assert((a == b) == 0);
  assert((a != b) == 3);
  assert((a < b) == 4);
  assert((b > a) == 5);
  assert((a <= c) == 6);
  assert((a >= c) == 7);

  X xs[] = {{3}, {1}, {2}};
  std::sort(xs, xs + 3);
  assert(xs[0].v == 1 && xs[1].v == 2 && xs[2].v == 3);

  Custom p{5}, q{2};
  Result r = p <=> q;
  assert(r.r == 3);

  Mixed m{4}, n{4};
  assert((m == n) == 9);
  assert((m <=> n).r == 0);
  Mixed ms[] = {{6}, {4}, {5}};
  std::sort(ms, ms + 3);
  assert(ms[0].v == 4 && ms[1].v == 5 && ms[2].v == 6);
  assert((ms[0] < ms[1]) == 8);
  return 0;
}
