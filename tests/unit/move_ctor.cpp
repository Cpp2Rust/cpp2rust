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

struct ThrowingMove {
  int v;
  int copies;
  int moves;
  ThrowingMove(int v) : v(v), copies(0), moves(0) {}
  ThrowingMove(const ThrowingMove &o)
      : v(o.v), copies(o.copies + 1), moves(o.moves) {}
  ThrowingMove(ThrowingMove &&o)
      : v(o.v), copies(o.copies), moves(o.moves + 1) {
    o.v = 0;
  }
};

struct NoexceptMove {
  int v;
  int copies;
  int moves;
  NoexceptMove(int v) : v(v), copies(0), moves(0) {}
  NoexceptMove(const NoexceptMove &o)
      : v(o.v), copies(o.copies + 1), moves(o.moves) {}
  NoexceptMove(NoexceptMove &&o) noexcept
      : v(o.v), copies(o.copies), moves(o.moves + 1) {
    o.v = 0;
  }
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

  ThrowingMove t(1);
  ThrowingMove t1(std::move_if_noexcept(t));
  assert(t1.v == 1);
  assert(t1.copies == 1);
  assert(t1.moves == 0);
  assert(t.v == 1);

  NoexceptMove n(2);
  NoexceptMove n1(std::move_if_noexcept(n));
  assert(n1.v == 2);
  assert(n1.copies == 0);
  assert(n1.moves == 1);
  assert(n.v == 0);

  MoveOnly g(3);
  MoveOnly g1(std::move_if_noexcept(g));
  assert(g1.v == 3);
  assert(g.v == 0);

  return 0;
}
