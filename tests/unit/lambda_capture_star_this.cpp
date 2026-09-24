#include <assert.h>

struct S {
  int n;
  int twice() const { return n * 2; }

  int modify_copy() {
    auto f = [*this]() mutable {
      n += 10;
      return n;
    };
    int r = f();
    return r * 100 + n;
  }

  int snapshot() {
    auto f = [*this]() { return twice(); };
    n = 99;
    return f();
  }

  int mixed(int k) {
    auto f = [=, *this]() { return n + k; };
    n = 0;
    return f();
  }
};

int main() {
  S s = {1};
  assert(s.modify_copy() == 1101);
  assert(s.n == 1);

  assert(s.snapshot() == 2);
  assert(s.n == 99);

  assert(s.mixed(1) == 100);
  assert(s.n == 0);

  return 0;
}
