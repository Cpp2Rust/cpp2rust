#include <cassert>

struct S {
  int v;
};

bool operator==(const S &a, const S &b) { return a.v == b.v; }
bool operator!=(const S &a, const S &b) { return a.v != b.v; }
bool operator<(const S &a, const S &b) { return a.v < b.v; }
bool operator>(const S &a, const S &b) { return a.v > b.v; }
bool operator<=(const S &a, const S &b) { return a.v <= b.v; }
bool operator>=(const S &a, const S &b) { return a.v >= b.v; }
bool operator<(const S &a, int b) { return a.v < b; }
bool operator<(int a, const S &b) { return a < b.v; }

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
  assert(0 < a);
  return 0;
}
