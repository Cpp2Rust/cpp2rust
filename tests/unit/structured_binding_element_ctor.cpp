// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>
#include <utility>

static int copies = 0;
static int moves = 0;

struct Elem {
  int v;

  Elem(int v) : v(v) {}
  Elem(const Elem &other) : v(other.v + 100) { ++copies; }
  Elem(Elem &&other) : v(other.v + 1000) {
    other.v = -1;
    ++moves;
  }
};

struct Two {
  Elem a;
  Elem b;
};

static Two make_two() { return Two{Elem(1), Elem(2)}; }

int main() {
  Two t{Elem(1), Elem(2)};
  int base_copies = copies;
  int base_moves = moves;

  auto [a, b] = t;
  assert(copies == base_copies + 2);
  assert(moves == base_moves);
  assert(a.v == 101);
  assert(b.v == 102);

  auto &[ra, rb] = t;
  assert(copies == base_copies + 2);
  assert(ra.v == 1);
  ra.v = 50;
  assert(t.a.v == 50);

  auto [ma, mb] = std::move(t);
  assert(moves == base_moves + 2);
  assert(ma.v == 1050);
  assert(mb.v == 1002);
  assert(t.a.v == -1);

  int before_copies = copies;
  int before_moves = moves;
  auto [pa, pb] = make_two();
  assert(copies == before_copies);
  assert(moves == before_moves);
  assert(pa.v == 1);

  std::pair<Elem, Elem> p{Elem(7), Elem(8)};
  before_copies = copies;
  auto [x, y] = p;
  assert(copies == before_copies + 2);
  assert(x.v == p.first.v + 100);
  assert(y.v == p.second.v + 100);
  return 0;
}
