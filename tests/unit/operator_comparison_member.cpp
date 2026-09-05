#include <cassert>

struct S {
  int v;
  bool operator==(const S &o) const { return v == o.v; }
  bool operator!=(const S &o) const { return v != o.v; }
  bool operator<(const S &o) const { return v < o.v; }
  bool operator>(const S &o) const { return v > o.v; }
  bool operator<=(const S &o) const { return v <= o.v; }
  bool operator>=(const S &o) const { return v >= o.v; }
  bool operator<(int o) const { return v < o; }
};

int main() {
  S a{1}, b{2}, c{1};
  assert(a == c);
  assert(a != b);
  assert(a < b);
  assert(b > a);
  assert(a <= c);
  assert(a >= c);
  assert(!(b < a));
  assert(a < 5);
  return 0;
}
