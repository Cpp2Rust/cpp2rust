#include <cassert>
#include <cstddef>

struct Base {
  int *buf;
  size_t n;
  Base(int *b, size_t n) : buf(b), n(n) {}
};

struct Derived : Base {
  using Base::Base;
};

size_t count(const Base &b) { return b.n; }
int first(Base *p) { return p->buf[0]; }

int main() {
  int arr[] = {7, 8, 9};
  Derived d(arr, 3);
  assert(count(d) == 3);
  assert(first(&d) == 7);
  Base copy = d;
  assert(copy.n == 3);
  assert(copy.buf == arr);
  return 0;
}
