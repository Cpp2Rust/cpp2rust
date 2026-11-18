// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.pre.incr]/1: the operand of prefix ++ is modified by adding 1;
// the result is the updated operand, and it is an lvalue.
namespace p1_result_is_updated_operand {
struct Counter {
  int n;
};

void test() {
  int x = 0;
  int &r = ++x;
  r = 5;
  assert(x == 5);
  assert(&r == &x);
  (++x) = 7;
  assert(x == 7);
  int y = ++x;
  assert(y == 8 && x == 8);
  int &s = r;
  (++s) = 1;
  assert(x == 1);
  int t = ++s;
  assert(t == 2 && x == 2);
  int a[3] = {1, 2, 3};
  int *p = a;
  int *&q = ++p;
  assert(&q == &p);
  assert(*q == 2);
  assert(*++p == 3);
  Counter c{0};
  Counter *pc = &c;
  int v = ++c.n;
  assert(v == 1 && pc->n == 1);
  (++c.n) = 4;
  assert(pc->n == 4);
  int sum = 0;
  for (int k = 0; k < 3; ++k) {
    sum += ++y;
  }
  assert(sum == 30 && y == 11);
}
} // namespace p1_result_is_updated_operand

// N4950 [expr.pre.incr]/2: the operand of prefix -- is modified by subtracting
// 1; the properties of its result are otherwise those of prefix ++.
namespace p2_decrement_result_is_updated_operand {
void test() {
  unsigned counter = 0;
  unsigned &pre = --counter;
  assert(&pre == &counter && counter == ~0u);
  int x = 5;
  int &r = x;
  --r;
  assert(x == 4);
  (--r) = 9;
  assert(x == 9);
  int a[3] = {1, 2, 3};
  int *p = a + 2;
  int *n = --p;
  assert(*n == 2 && n == p);
}
} // namespace p2_decrement_result_is_updated_operand

int main() {
  p1_result_is_updated_operand::test();
  p2_decrement_result_is_updated_operand::test();
  return 0;
}
