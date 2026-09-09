#include <cassert>
#include <utility>
#include <vector>

struct MoveOnly {
  int v;
  MoveOnly(int v) : v(v) {}
  MoveOnly(const MoveOnly &) = delete;
  MoveOnly(MoveOnly &&o) : v(o.v) { o.v = 0; }
};

struct ConstMove {
  int mark;
  ConstMove() : mark(0) {}
  ConstMove(ConstMove &&o) : mark(o.mark + 1) {}
  ConstMove(const ConstMove &&o) : mark(o.mark + 10) {}
};

static int by_value(MoveOnly m) { return m.v; }

static MoveOnly make(int v) {
  MoveOnly m(v);
  return std::move(m);
}

int main() {
  MoveOnly a(1);
  MoveOnly b(std::move(a));
  assert(b.v == 1);
  assert(a.v == 0);

  MoveOnly c = std::move(b);
  assert(c.v == 1);
  assert(b.v == 0);

  MoveOnly d{std::move(c)};
  assert(d.v == 1);
  assert(c.v == 0);

  MoveOnly e = make(5);
  assert(e.v == 5);

  assert(by_value(MoveOnly(6)) == 6);
  assert(by_value(std::move(e)) == 5);
  assert(e.v == 0);

  std::vector<MoveOnly> vec;
  vec.push_back(MoveOnly(7));
  MoveOnly f(8);
  vec.push_back(std::move(f));
  assert(vec[0].v == 7 && vec[1].v == 8);
  assert(f.v == 0);

  ConstMove m;
  ConstMove m1(std::move(m));
  const ConstMove cm;
  ConstMove m2(std::move(cm));
  assert(m1.mark == 1);
  assert(m2.mark == 10);
  return 0;
}
