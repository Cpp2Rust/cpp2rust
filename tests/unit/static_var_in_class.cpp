#include <assert.h>

class C {
  static const int inner_const = 1;

public:
  int get() { return inner_const; }
};

struct S {
  static const int inner_const = 2;
};

int main() {
  C c;
  assert(c.get() == 1);
  assert(S::inner_const == 2);
  return 0;
}
