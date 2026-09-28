#include <assert.h>

class C {
  static const int inner_const = 1;

public:
  int get() { return inner_const; }
};

struct S {
  static const int inner_const = 2;
  static int counter;
  enum { kValue = 3 };
};

int S::counter = 10;

int main() {
  C c;
  assert(c.get() == 1);
  assert(S::inner_const == 2);
  S s;
  S *p = &s;
  assert(s.inner_const == 2);
  assert(p->inner_const == 2);
  assert(s.kValue == 3);
  assert(p->kValue == 3);
  s.counter = 20;
  assert(S::counter == 20);
  p->counter += 5;
  assert(s.counter == 25);
  assert(S::counter == 25);
  return 0;
}
