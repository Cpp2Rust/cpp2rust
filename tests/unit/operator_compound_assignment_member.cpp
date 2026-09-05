#include <cassert>

struct S {
  unsigned v;
  S &operator=(unsigned n) {
    v = n;
    return *this;
  }
  S &operator+=(const S &o) {
    v += o.v;
    return *this;
  }
  S &operator-=(const S &o) {
    v -= o.v;
    return *this;
  }
  S &operator*=(const S &o) {
    v *= o.v;
    return *this;
  }
  S &operator/=(const S &o) {
    v /= o.v;
    return *this;
  }
  S &operator%=(const S &o) {
    v %= o.v;
    return *this;
  }
  S &operator&=(const S &o) {
    v &= o.v;
    return *this;
  }
  S &operator|=(const S &o) {
    v |= o.v;
    return *this;
  }
  S &operator^=(const S &o) {
    v ^= o.v;
    return *this;
  }
  S &operator<<=(int n) {
    v <<= n;
    return *this;
  }
  S &operator>>=(int n) {
    v >>= n;
    return *this;
  }
};

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
  a = 3;
  assert(a.v == 3);
  a <<= 2;
  assert(a.v == 12);
  a >>= 1;
  assert(a.v == 6);
  (a += b) += b;
  assert(a.v == 14);
  S c{0};
  c = a = 1;
  assert(a.v == 1);
  assert(c.v == 1);
  return 0;
}
