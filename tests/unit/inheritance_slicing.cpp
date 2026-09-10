#include <cassert>

struct Base {
  int a;
  int copies;
  Base(int x) : a(x), copies(0) {}
  Base(const Base &o) : a(o.a), copies(o.copies + 1) {}
  Base &operator=(const Base &o) {
    a = o.a;
    copies = o.copies + 1;
    return *this;
  }
};

struct Derived : Base {
  int b;
  Derived(int x) : Base(x), b(x * 10) {}
};

int take(Base v) { return v.a + v.copies; }

Base make(int x) {
  Derived d(x);
  return d;
}

int main() {
  Derived d(1);
  Base init = d;
  assert(init.a == 1);
  assert(init.copies == 1);

  Base assigned(9);
  assigned = d;
  assert(assigned.a == 1);
  assert(assigned.copies == 1);

  assert(take(d) == 2);

  Base made = make(3);
  assert(made.a == 3);
  assert(made.copies >= 1);

  d.a = 7;
  assert(init.a == 1);
  assert(assigned.a == 1);
  return 0;
}
