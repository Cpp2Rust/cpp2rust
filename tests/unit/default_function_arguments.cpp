#include <cassert>

int foo(int a, int b = 10) { return a + b; }

bool baz(int *a, int *b = nullptr) { return a == b; }

struct Bar {
  int v;
  Bar(int v = 1) : v(v) {}
};

int main() {
  assert(foo(1) == 11);
  assert(foo(1, 2) == 3);

  int a = 0;
  assert(baz(&a) == false);
  assert(baz(&a, &a) == true);

  Bar b;
  assert(b.v == 1);
  assert(Bar(2).v == 2);

  Bar arr[3] = {};
  assert(arr[0].v == 1);
  assert(arr[2].v == 1);

  return 0;
}
