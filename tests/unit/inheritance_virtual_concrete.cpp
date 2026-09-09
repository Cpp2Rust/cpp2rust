#include <cassert>

struct Base {
  int v = 1;
  virtual int get() { return v; }
};

struct Derived : Base {
  int w = 2;
  int get() override { return v + w; }
};

int main() {
  Base b;
  Derived d;
  Base *p = &d;
  assert(b.get() == 1);
  assert(p->get() == 3);
  assert(p->v == 1);
  return 0;
}
