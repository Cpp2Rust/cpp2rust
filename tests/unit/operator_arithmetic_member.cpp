#include <cassert>

struct S {
  int v;
  S operator+(const S &o) const { return {v + o.v}; }
  S operator-(const S &o) const { return {v - o.v}; }
  S operator*(const S &o) const { return {v * o.v}; }
  S operator/(const S &o) const { return {v / o.v}; }
  S operator%(const S &o) const { return {v % o.v}; }
  S operator+() const { return {v}; }
  S operator-() const { return {-v}; }
  S &operator++() {
    ++v;
    return *this;
  }
  S operator++(int) {
    S old = *this;
    ++v;
    return old;
  }
  S &operator--() {
    --v;
    return *this;
  }
  S operator--(int) {
    S old = *this;
    --v;
    return old;
  }
};

int main() {
  S a{7}, b{2};
  assert((a + b).v == 9);
  assert((a - b).v == 5);
  assert((a * b).v == 14);
  assert((a / b).v == 3);
  assert((a % b).v == 1);
  assert((+a).v == 7);
  assert((-a).v == -7);
  assert((++a).v == 8);
  assert((a++).v == 8);
  assert(a.v == 9);
  assert((--a).v == 8);
  assert((a--).v == 8);
  assert(a.v == 7);
  assert((++(++a)).v == 9);
  return 0;
}
