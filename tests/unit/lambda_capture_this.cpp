// no-compile: refcount
#include <assert.h>

struct S {
  int n;
  int step;
  void add(int k) { n += k; }
  int scaled() const { return n * step; }
  void bump(int by) {
    auto inc = [this](int k) { n += k; };
    inc(by);
    inc(by);
  }
  void bump_via_method(int by) {
    auto inc = [this](int k) { add(k); };
    inc(by);
  }
  int read_scaled() const {
    auto get = [this]() { return scaled(); };
    return get();
  }
};

int main() {
  S s = {0, 2};
  s.bump(3);
  assert(s.n == 6);
  s.bump_via_method(4);
  assert(s.n == 10);
  assert(s.read_scaled() == 20);
  return 0;
}
