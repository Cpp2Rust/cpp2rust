#include <cassert>

struct S {
  unsigned v;
};

S operator~(const S &a) { return {~a.v}; }
S operator&(const S &a, const S &b) { return {a.v & b.v}; }
S operator|(const S &a, const S &b) { return {a.v | b.v}; }
S operator^(const S &a, const S &b) { return {a.v ^ b.v}; }
S operator<<(const S &a, int n) { return {a.v << n}; }
S operator>>(const S &a, int n) { return {a.v >> n}; }

int main() {
  S a{0b1100}, b{0b1010};
  assert((~a).v == ~0b1100u);
  assert((a & b).v == 0b1000);
  assert((a | b).v == 0b1110);
  assert((a ^ b).v == 0b0110);
  assert((a << 2).v == 0b110000);
  assert((a >> 2).v == 0b11);
  return 0;
}
