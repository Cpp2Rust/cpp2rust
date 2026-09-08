#include <cassert>

struct S {
  int v;
};

bool operator!(const S &a) { return a.v == 0; }
bool operator&&(const S &a, const S &b) { return a.v != 0 && b.v != 0; }
bool operator||(const S &a, const S &b) { return a.v != 0 || b.v != 0; }

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
