#include <cassert>
#include <new>
#include <utility>

enum Overload {
  kMutableOverload = 1,
  kConstOverload = 2,
};

struct S {
  int v;
  Overload f() { return kMutableOverload; }
  Overload f() const { return kConstOverload; }
  int &value_ref() { return v; }
  const int &value_ref() const { return v; }
};

static Overload g(S &) { return kMutableOverload; }

static Overload g(const S &) { return kConstOverload; }

int main() {
  S s{7};
  assert(s.f() == kMutableOverload);
  assert(std::as_const(s).f() == kConstOverload);

  assert(g(s) == kMutableOverload);
  assert(g(std::as_const(s)) == kConstOverload);

  s.value_ref() = 9;
  assert(s.v == 9);
  assert(std::as_const(s).value_ref() == 9);

  const S &cs = std::as_const(s);
  assert(cs.f() == kConstOverload);
  assert(cs.v == 9);

  S *p = std::launder(&s);
  p->v = 11;
  assert(s.v == 11);
  assert(std::launder(p)->f() == kMutableOverload);

  return 0;
}
