#include <cassert>

struct S {
  int v;
  bool operator!() const { return v == 0; }
  bool operator&&(const S &o) const { return v != 0 && o.v != 0; }
  bool operator||(const S &o) const { return v != 0 || o.v != 0; }
};

int main() {
  S t{1}, f{0};
  assert(!f);
  assert(!!t);
  assert(t && t);
  assert(!(t && f));
  assert(t || f);
  assert(!(f || f));
  return 0;
}
