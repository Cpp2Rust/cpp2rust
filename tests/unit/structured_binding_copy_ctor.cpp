// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>

static int copies = 0;

struct Counted {
  int x;
  int y;

  Counted(int x, int y) : x(x), y(y) {}
  Counted(const Counted &other) : x(other.x * 2), y(other.y * 2) { ++copies; }
};

static Counted make_counted() { return Counted(5, 6); }

int main() {
  Counted s(1, 2);

  auto [a, b] = s;
  assert(copies == 1);
  assert(a == 2);
  assert(b == 4);

  auto &[ra, rb] = s;
  assert(copies == 1);
  assert(ra == 1);
  assert(rb == 2);

  const auto &[ca, cb] = s;
  assert(copies == 1);
  assert(ca == 1);

  auto [pa, pb] = make_counted();
  assert(copies == 1);
  assert(pa == 5);
  assert(pb == 6);
  return 0;
}
