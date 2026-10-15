// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>
#include <utility>
#include <vector>

static int copies = 0;
static int moves = 0;

struct Movable {
  int x;
  int y;

  Movable(int x, int y) : x(x), y(y) {}
  Movable(const Movable &other) : x(other.x), y(other.y) { ++copies; }
  Movable(Movable &&other) : x(other.x), y(other.y) {
    other.x = 0;
    other.y = 0;
    ++moves;
  }
};

struct Holder {
  std::vector<int> xs;
  std::vector<int> ys;

  Holder extract() && { return {std::move(xs), std::move(ys)}; }
};

int main() {
  Movable m(3, 4);
  auto [a, b] = std::move(m);
  assert(moves == 1);
  assert(copies == 0);
  assert(a == 3);
  assert(b == 4);
  assert(m.x == 0);
  assert(m.y == 0);

  Holder h{{1, 2, 3}, {4, 5}};
  auto [ks, vs] = std::move(h).extract();
  assert(ks.size() == 3);
  assert(vs.size() == 2);
  assert(ks[2] == 3);
  assert(vs[0] == 4);
  assert(h.xs.empty());
  assert(h.ys.empty());
  return 0;
}
