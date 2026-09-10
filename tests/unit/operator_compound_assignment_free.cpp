#include <cassert>

struct S {
  unsigned v;
};

S &operator+=(S &a, const S &b) {
  a.v += b.v;
  return a;
}
S &operator-=(S &a, const S &b) {
  a.v -= b.v;
  return a;
}
S &operator*=(S &a, const S &b) {
  a.v *= b.v;
  return a;
}
S &operator/=(S &a, const S &b) {
  a.v /= b.v;
  return a;
}
S &operator%=(S &a, const S &b) {
  a.v %= b.v;
  return a;
}
S &operator&=(S &a, const S &b) {
  a.v &= b.v;
  return a;
}
S &operator|=(S &a, const S &b) {
  a.v |= b.v;
  return a;
}
S &operator^=(S &a, const S &b) {
  a.v ^= b.v;
  return a;
}
S &operator<<=(S &a, int n) {
  a.v <<= n;
  return a;
}
S &operator>>=(S &a, int n) {
  a.v >>= n;
  return a;
}

int main() {
  S a{6}, b{4};
  a += b;
  assert(a.v == 10);
  a -= b;
  assert(a.v == 6);
  a *= b;
  assert(a.v == 24);
  a /= b;
  assert(a.v == 6);
  a %= b;
  assert(a.v == 2);
  a |= b;
  assert(a.v == 6);
  a &= b;
  assert(a.v == 4);
  a ^= b;
  assert(a.v == 0);
  a.v = 3;
  a <<= 2;
  assert(a.v == 12);
  a >>= 1;
  assert(a.v == 6);
  (a += b) += b;
  assert(a.v == 14);
  return 0;
}
