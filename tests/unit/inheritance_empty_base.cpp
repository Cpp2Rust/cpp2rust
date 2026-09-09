#include <cassert>

struct Tag {};

struct View : Tag {
  int i;
};

struct Base {};
struct Derived : Base {};

Base *as_base(Derived *d) { return d; }

int main() {
  View v;
  v.i = 5;
  assert(v.i == 5);
  Derived d;
  Base *b = as_base(&d);
  assert(b == &d);
  return 0;
}
