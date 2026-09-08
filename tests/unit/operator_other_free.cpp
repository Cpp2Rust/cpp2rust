#include <cassert>

struct S {
  int v;
};

S operator,(const S &a, const S &b) { return {a.v * 10 + b.v}; }

int main() {
  S s{3}, t{4};
  assert((s, t).v == 34);
  assert((s, t, s).v == 343);
  return 0;
}
