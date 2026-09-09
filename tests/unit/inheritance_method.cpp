#include <cassert>

struct Base {
  char buf[8];
  void fill(char c, int n) {
    for (int i = 0; i < n; i++) {
      buf[i] = c;
    }
  }
};

struct Derived : Base {
  using Base::fill;
  bool run() {
    fill('x', 3);
    return buf[2] == 'x';
  }
};

int main() {
  Derived d;
  assert(d.run());
  d.fill('y', 1);
  assert(d.buf[0] == 'y');
  assert(d.buf[1] == 'x');
  return 0;
}
