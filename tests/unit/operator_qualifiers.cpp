#include <cassert>

struct S {
  int v;
  int operator+(int a) { return v + a; }
  int operator+(int a) const { return v + a + 1; }
  int operator+(int a) volatile { return v + a + 2; }
  int operator-(int a) & { return v - a; }
  int operator-(int a) && { return v - a - 1; }
  int operator*(int a) const & { return v * a; }
  int operator*(int a) const && { return v * a * 2; }
  int operator[](int i) & { return v + i; }
  int operator[](int i) const & { return v + i + 100; }
};

int main() {
  S s{10};
  const S cs{10};
  volatile S vs{10};
  assert(s + 1 == 11);
  assert(cs + 1 == 12);
  assert(vs + 1 == 13);
  assert(s - 1 == 9);
  assert(S{10} - 1 == 8);
  assert(s * 3 == 30);
  assert(cs * 3 == 30);
  assert(S{10} * 3 == 60);
  assert(s[2] == 12);
  assert(cs[2] == 112);
  return 0;
}
