#include <atomic>
#include <cassert>

struct Pair {
  int x;
  int y;
};

int main() {
  std::atomic<Pair> a{};
  Pair v = a;
  assert(v.x == 0);
  assert(v.y == 0);
  return 0;
}
