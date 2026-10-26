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
};

int by_value(S s) {
  // The parameter is destroyed when the function returns.
  assert(destroyed == 0);
  return s.v;
}

int with_default(S s = S{5}) { return s.v; }

struct Holder {
  int v;
  explicit Holder(S s) : v(s.v) {}
  int add(S s) const { return v + s.v; }
};

int main() {
  {
    S s{1};
    assert(by_value(s) == 1);
    assert(destroyed == 1 && last == 1);
  }
  assert(destroyed == 2);
  destroyed = 0;

  assert(by_value(S{2}) == 2);
  assert(destroyed == 1 && last == 2);
  destroyed = 0;

  assert(with_default() == 5);
  assert(destroyed == 1 && last == 5);
  destroyed = 0;

  {
    Holder h(S{3});
    assert(destroyed == 1 && last == 3);
    assert(h.add(S{4}) == 7);
    assert(destroyed == 2 && last == 4);
  }
  assert(destroyed == 2);
  return 0;
}
