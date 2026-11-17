// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.ass]/1: the result of an assignment is an lvalue referring to
// the left operand.
namespace p1_result_is_lvalue {
int **next(int **pp, int &calls) {
  ++calls;
  return pp;
}

void test() {
  int a = 1;
  int b = 42;
  int &r = (a = b);
  assert(&r == &a);
  (a = b) = 5;
  assert(a == 5);
  assert(b == 42);
  (a += 2) *= 3;
  assert(a == 21);
  int c = 0;
  (c = b) = 3;
  assert(c == 3);
  (c -= 1) += 10;
  assert(c == 12);
  int arr[3] = {1, 2, 3};
  int i = 0;
  (arr[i++] = 9) = 7;
  assert(i == 1);
  assert(arr[0] == 7);
  assert(arr[1] == 2);
  int *p = arr;
  (p += 1) += 1;
  assert(p == arr + 2);
  int *q = arr;
  int calls = 0;
  *next(&q, calls) += 1;
  assert(calls == 1);
  assert(q == arr + 1);
}
} // namespace p1_result_is_lvalue

int main() {
  p1_result_is_lvalue::test();
  return 0;
}
