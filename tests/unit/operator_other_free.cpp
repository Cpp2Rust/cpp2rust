#include <cassert>

struct S {
  int v;
};

S operator,(const S &a, const S &b) { return {a.v * 10 + b.v}; }

long operator""_k(unsigned long long v) { return v * 1000; }

double operator""_half(long double v) { return v / 2; }

int main() {
  S s{3}, t{4};
  assert((s, t).v == 34);
  assert((s, t, s).v == 343);

  assert(2_k == 2000);
  assert(3.0_half == 1.5);
  assert(operator""_k(4) == 4000);
  return 0;
}
