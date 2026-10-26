// XFAIL: refcount, unsafe
#include <cassert>

int destroyed = 0;
int last = 0;

struct S {
  int v;
  ~S() {
    destroyed++;
    last = v;
  }
  int get() const { return v; }
};

struct Outer {
  S inner;
  int w;
};

S make(int v) { return S{v}; }

Outer make_outer() { return Outer{{7}, 8}; }

int use(const S &s) {
  // The temporary is still alive while the call runs.
  assert(destroyed == 0);
  return s.v;
}

int value(const S &s) { return s.v; }

int main() {
  make(1);
  assert(destroyed == 1 && last == 1);
  destroyed = 0;

  (void)make(2);
  assert(destroyed == 1 && last == 2);
  destroyed = 0;

  int x = make(3).v;
  assert(x == 3 && destroyed == 1 && last == 3);
  destroyed = 0;

  x = make(4).get();
  assert(x == 4 && destroyed == 1 && last == 4);
  destroyed = 0;

  x = make_outer().inner.v;
  assert(x == 7 && destroyed == 1 && last == 7);
  destroyed = 0;

  x = S{5}.v;
  assert(x == 5 && destroyed == 1);
  destroyed = 0;

  x = use(make(6));
  assert(x == 6 && destroyed == 1 && last == 6);
  destroyed = 0;

  x = value(S{1}) + value(S{2});
  assert(x == 3 && destroyed == 2);
  destroyed = 0;

  x = (x == 3 ? make(9) : S{10}).v;
  assert(x == 9 && destroyed == 1 && last == 9);
  destroyed = 0;

  {
    // The temporary lives as long as the reference bound to it.
    const S &r = make(11);
    assert(destroyed == 0 && r.v == 11);
  }
  assert(destroyed == 1 && last == 11);
  destroyed = 0;

  {
    // The assigned temporary is copied, and then destroyed.
    S s{12};
    s = make(13);
    assert(destroyed == 1 && last == 13 && s.v == 13);
  }
  assert(destroyed == 2 && last == 13);
  return 0;
}
