#include <assert.h>

struct Counter {
  int n = 0;
  void bump(int by) {
    auto inc = [this](int k) { n += k; };
    inc(by);
    inc(by);
  }
};

int main() {
  Counter c;
  c.bump(3);
  assert(c.n == 6);
  return 0;
}
