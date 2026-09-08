#include <cassert>

struct S {
  int v;
  bool operator==(int o) const { return v == o; }
  bool operator!=(int o) const { return v != o; }
  bool operator<(int o) const { return v < o; }
  bool operator>(double o) const { return v > o; }
  bool operator<=(long o) const { return v <= o; }
  bool operator>=(const char *o) const { return v >= *o - '0'; }
};

bool operator==(int a, const S &b) { return a == b.v; }
bool operator!=(int a, const S &b) { return a != b.v; }
bool operator<(int a, const S &b) { return a < b.v; }
bool operator>(double a, const S &b) { return a > b.v; }
bool operator<=(long a, const S &b) { return a <= b.v; }
bool operator>=(const char *a, const S &b) { return *a - '0' >= b.v; }
bool operator<(S &a, int b) { return a.v + 1 < b; }

int main() {
  S s{5};
  const S &cs = s;
  assert(cs == 5);
  assert(cs != 4);
  assert(cs < 6);
  assert(cs > 4.5);
  assert(cs <= 5L);
  assert(cs >= "3");
  assert(5 == s);
  assert(4 != s);
  assert(4 < s);
  assert(5.5 > s);
  assert(5L <= s);
  assert("7" >= s);
  assert(s < 7);
  assert(!(s < 6));
  return 0;
}
