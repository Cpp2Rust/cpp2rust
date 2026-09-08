#include <cassert>

struct S {
  int v;
};

S operator+(const S &a, const S &b) { return {a.v + b.v}; }
S operator-(const S &a, const S &b) { return {a.v - b.v}; }
S operator*(const S &a, const S &b) { return {a.v * b.v}; }
S operator/(const S &a, const S &b) { return {a.v / b.v}; }
S operator%(const S &a, const S &b) { return {a.v % b.v}; }
S operator+(const S &a) { return {a.v}; }
S operator-(const S &a) { return {-a.v}; }
S &operator++(S &a) {
  ++a.v;
  return a;
}
S operator++(S &a, int) {
  S old = a;
  ++a.v;
  return old;
}
S &operator--(S &a) {
  --a.v;
  return a;
}
S operator--(S &a, int) {
  S old = a;
  --a.v;
  return old;
}
S operator+(const S &a, int b) { return {a.v + b}; }
S operator+(int a, const S &b) { return {a + b.v}; }

int main() {
  S a{7}, b{2};
  assert((a + b).v == 9);
  assert((a - b).v == 5);
  assert((a * b).v == 14);
  assert((a / b).v == 3);
  assert((a % b).v == 1);
  assert((+a).v == 7);
  assert((-a).v == -7);
  assert((++a).v == 8);
  assert((a++).v == 8);
  assert(a.v == 9);
  assert((--a).v == 8);
  assert((a--).v == 8);
  assert(a.v == 7);
  assert((a + 1).v == 8);
  assert((1 + a).v == 8);
  return 0;
}
