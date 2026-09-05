#include <cassert>

struct S {
  unsigned v;
  S operator~() const { return {~v}; }
  S operator&(const S &o) const { return {v & o.v}; }
  S operator|(const S &o) const { return {v | o.v}; }
  S operator^(const S &o) const { return {v ^ o.v}; }
  S operator<<(int n) const { return {v << n}; }
  S operator>>(int n) const { return {v >> n}; }
};

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
