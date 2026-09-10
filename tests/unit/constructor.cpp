#include <cassert>

static int total = 0;

struct S {
  int v;
  int const_method() const { return v * 2; }
  void mut_method() { v += 1; }

  S(int init) : v(init) {
    mut_method();
    total += const_method();
  }

  ~S() {
    mut_method();
    total += this->const_method();
  }
};

int main() {
  {
    S s(3);
    assert(s.v == 4);
    assert(total == 8);
  }
  assert(total == 18);
  return 0;
}
