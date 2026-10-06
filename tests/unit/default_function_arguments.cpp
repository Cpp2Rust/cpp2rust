#include <cassert>

int foo(int a, int b = 10) { return a + b; }

bool baz(int *a, int *b = nullptr) { return a == b; }

struct Bar {
  int v;
  Bar(int v = 1) : v(v) {}
};

int counter = 0;

int next() { return ++counter; }

int lazy(int x = next()) { return x; }

int by_ref(const Bar &b = Bar(7)) { return b.v; }

Bar global_bar(9);

int by_global_ref(const Bar &b = global_bar) { return b.v; }

template <typename T> struct Holder {
  T v;
  explicit Holder(T v = 0) : v(v) {}
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

  assert(lazy(5) == 5);
  assert(counter == 0);
  assert(lazy() == 1);
  assert(counter == 1);

  assert(by_ref() == 7);
  assert(by_ref(Bar(3)) == 3);
  assert(by_global_ref() == 9);
  assert(by_global_ref(Bar(4)) == 4);

  Holder<int> h(4);
  assert(h.v == 4);

  return 0;
}
