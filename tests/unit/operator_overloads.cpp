#include <cassert>

struct S {
  int v;
  int operator==(int o) const { return v == o ? 1 : 0; }
  int operator==(long o) const { return v == o ? 2 : 0; }
  int operator==(double o) const { return v == o ? 3 : 0; }
  int operator+(const S &o) const { return v + o.v; }
  int operator-(S o) const { return v - o.v; }
  int operator*(const S &o) const { return v * o.v; }
  int operator*(int o) const { return v * o + 1; }
};

int operator/(const S &a, const S &b) { return a.v / b.v; }
int operator/(S a, int b) { return a.v / b + 1; }
int operator%(S a, S b) { return a.v % b.v; }
int operator%(const S &a, int b) { return a.v % b + 1; }
int operator==(int a, S b) { return a == b.v ? 4 : 0; }
int operator==(long a, const S &b) { return a == b.v ? 5 : 0; }

int main() {
  S s{6}, t{4};
  assert((s == 6) == 1);
  assert((s == 6L) == 2);
  assert((s == 6.0) == 3);
  assert((s == 7) == 0);
  assert(s + t == 10);
  assert(s - t == 2);
  assert(s * t == 24);
  assert(s * 2 == 13);
  assert(s / t == 1);
  assert(s / 4 == 2);
  assert(s % t == 2);
  assert(s % 4 == 3);
  assert((6 == s) == 4);
  assert((6L == s) == 5);
  return 0;
}
