// translation-fail
#include <cassert>
#include <utility>

struct NoCopy {
  int v;
  NoCopy(int v) : v(v) {}
  NoCopy(const NoCopy &) = delete;
  NoCopy &operator=(const NoCopy &) = delete;
  NoCopy(NoCopy &&o) : v(o.v) { o.v = 0; }
  NoCopy &operator=(NoCopy &&o) {
    v = o.v;
    o.v = 0;
    return *this;
  }
};

class PrivateCopy {
  PrivateCopy(const PrivateCopy &);
  PrivateCopy &operator=(const PrivateCopy &);

public:
  int v;
  PrivateCopy() : v(0) {}
  PrivateCopy(PrivateCopy &&o) : v(o.v) { o.v = 0; }
  PrivateCopy &operator=(PrivateCopy &&o) {
    v = o.v;
    o.v = 0;
    return *this;
  }
};

struct Immovable {
  int v;
  Immovable() : v(0) {}
  Immovable(const Immovable &) = delete;
  Immovable(Immovable &&) = delete;
  Immovable &operator=(const Immovable &) = delete;
  Immovable &operator=(Immovable &&) = delete;
};

struct Container {
  NoCopy inner;
  int tag;
};

static void bump(NoCopy *p) { p->v++; }
static void bump_ref(Immovable &r) { r.v++; }

int main() {
  NoCopy a(1);
  NoCopy b(std::move(a));
  assert(b.v == 1 && a.v == 0);
  a = std::move(b);
  assert(a.v == 1 && b.v == 0);
  bump(&a);
  assert(a.v == 2);

  PrivateCopy p;
  p.v = 3;
  PrivateCopy q(std::move(p));
  assert(q.v == 3 && p.v == 0);
  p = std::move(q);
  assert(p.v == 3 && q.v == 0);

  Immovable im;
  im.v = 4;
  bump_ref(im);
  Immovable *pim = &im;
  assert(pim->v == 5);

  Container c{NoCopy(6), 7};
  Container d(std::move(c));
  assert(d.inner.v == 6 && d.tag == 7 && c.inner.v == 0);
  return 0;
}
