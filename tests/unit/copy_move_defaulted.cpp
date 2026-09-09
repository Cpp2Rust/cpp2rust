// translation-fail
#include <cassert>
#include <utility>
#include <vector>

struct Inner {
  int x;
};

struct Explicit {
  int v;
  Inner inner;
  int arr[2];
  Explicit(int v) : v(v), inner{v * 10}, arr{v, v + 1} {}
  Explicit(const Explicit &) = default;
  Explicit(Explicit &&) = default;
  Explicit &operator=(const Explicit &) = default;
  Explicit &operator=(Explicit &&) = default;
  ~Explicit() {}
};

struct Implicit {
  int v;
  Inner inner;
  int arr[2];
};

struct DefaultCopyUserMove {
  int v;
  DefaultCopyUserMove(int v) : v(v) {}
  DefaultCopyUserMove(const DefaultCopyUserMove &) = default;
  DefaultCopyUserMove(DefaultCopyUserMove &&o) : v(o.v) { o.v = 0; }
  DefaultCopyUserMove &operator=(const DefaultCopyUserMove &) = default;
  DefaultCopyUserMove &operator=(DefaultCopyUserMove &&o) {
    v = o.v;
    o.v = 0;
    return *this;
  }
};

struct UserCopyDefaultMove {
  int v;
  UserCopyDefaultMove(int v) : v(v) {}
  UserCopyDefaultMove(const UserCopyDefaultMove &o) : v(o.v + 100) {}
  UserCopyDefaultMove(UserCopyDefaultMove &&) = default;
  UserCopyDefaultMove &operator=(const UserCopyDefaultMove &o) {
    v = o.v + 100;
    return *this;
  }
  UserCopyDefaultMove &operator=(UserCopyDefaultMove &&) = default;
};

static bool same(const Explicit &a, const Explicit &b) {
  return a.v == b.v && a.inner.x == b.inner.x && a.arr[0] == b.arr[0] &&
         a.arr[1] == b.arr[1];
}

int main() {
  Explicit a(1);
  Explicit b = a;
  Explicit c(a);
  Explicit d(std::move(a));
  assert(same(b, a) && same(c, a) && same(d, a));

  Explicit e(2), f(3);
  e = b;
  f = std::move(c);
  assert(same(e, b) && same(f, c));
  Explicit g(4);
  g = e = f;
  assert(same(g, f) && same(e, f));

  Implicit i{5, {50}, {5, 6}};
  Implicit j = i;
  Implicit k = std::move(i);
  assert(j.v == 5 && j.inner.x == 50 && j.arr[1] == 6);
  assert(i.v == 5 && k.v == 5);
  Implicit l{0, {0}, {0, 0}};
  l = j;
  assert(l.v == 5 && l.inner.x == 50 && l.arr[0] == 5);

  std::vector<Explicit> vec;
  vec.push_back(b);
  vec.push_back(Explicit(9));
  assert(vec[0].v == 1 && vec[1].v == 9);

  DefaultCopyUserMove m(7);
  DefaultCopyUserMove m1 = m;
  DefaultCopyUserMove m2 = std::move(m);
  assert(m1.v == 7 && m2.v == 7 && m.v == 0);
  DefaultCopyUserMove m3(1), m4(1);
  m3 = m1;
  m4 = std::move(m1);
  assert(m3.v == 7 && m4.v == 7 && m1.v == 0);

  UserCopyDefaultMove u(8);
  UserCopyDefaultMove u1 = u;
  UserCopyDefaultMove u2 = std::move(u);
  assert(u1.v == 108 && u2.v == 8 && u.v == 8);
  UserCopyDefaultMove u3(1), u4(1);
  u3 = u2;
  u4 = std::move(u2);
  assert(u3.v == 108 && u4.v == 8);
  return 0;
}
