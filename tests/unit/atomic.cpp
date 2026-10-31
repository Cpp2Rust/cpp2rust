#include <atomic>
#include <cassert>

struct Counter {
  int n;
};

int main() {
  std::atomic<Counter> a{};
  Counter v = a;
  assert(v.n == 0);
  return 0;
}
