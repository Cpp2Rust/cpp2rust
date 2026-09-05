#include <cassert>

struct S {
  int v;
  int operator()() const { return v; }
  int operator()(int a) const { return v + a; }
  int operator()(int a, int b) const { return v + a + b; }
  S operator,(const S &o) const { return {v * 10 + o.v}; }
  operator int() const { return v; }
  explicit operator bool() const { return v != 0; }
};

int main() {
  S s{3}, t{4};
  assert(s() == 3);
  assert(s(1) == 4);
  assert(s(1, 2) == 6);
  assert((s, t).v == 34);
  int i = s;
  assert(i == 3);
  assert(s + 1 == 4);
  if (s) {
    assert(static_cast<bool>(s));
  } else {
    assert(false);
  }
  S z{0};
  assert(!z);
  return 0;
}
