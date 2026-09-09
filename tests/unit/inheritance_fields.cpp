#include <cassert>
#include <cstddef>

struct Base {
  int *buf;
  size_t n;
  Base(int *b, size_t n) : buf(b), n(n) {}
};

struct Derived : Base {
  using Base::Base;
  int *begin() { return buf; }
  int *end() { return buf + n; }
};

int main() {
  int arr[] = {1, 2, 3};
  Derived d(arr, 3);
  assert(d.n == 3);
  assert(d.buf[1] == 2);
  assert(*d.begin() == 1);
  assert(d.end() - d.begin() == 3);
  return 0;
}
