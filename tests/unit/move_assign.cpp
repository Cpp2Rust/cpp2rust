#include <cassert>
#include <utility>
#include <vector>

struct MoveOnly {
  int v;
  MoveOnly(int v) : v(v) {}
  MoveOnly(const MoveOnly &) = delete;
  MoveOnly(MoveOnly &&o) : v(o.v) { o.v = 0; }
  MoveOnly &operator=(const MoveOnly &) = delete;
  MoveOnly &operator=(MoveOnly &&o) {
    if (this == &o) {
      return *this;
    }
    v = o.v;
    o.v = 0;
    return *this;
  }
};

struct ConstMoveAssign {
  int mark;
  ConstMoveAssign() : mark(0) {}
  ConstMoveAssign &operator=(ConstMoveAssign &&o) {
    mark = o.mark + 1;
    return *this;
  }
  ConstMoveAssign &operator=(const ConstMoveAssign &&o) {
    mark = o.mark + 10;
    return *this;
  }
};

static MoveOnly make(int v) {
  MoveOnly m(v);
  return std::move(m);
}

int main() {
  MoveOnly a(1), b(2), c(3);
  a = std::move(b);
  assert(a.v == 2);
  assert(b.v == 0);

  b = MoveOnly(3);
  c = std::move(a = std::move(b));
  assert(b.v == 0 && a.v == 0 && c.v == 3);

  a = MoveOnly(5);
  assert(a.v == 5);

  a = make(6);
  assert(a.v == 6);

  a = std::move(a);
  assert(a.v == 6);

  std::vector<MoveOnly> vec;
  vec.push_back(MoveOnly(7));
  MoveOnly d(8);
  vec[0] = std::move(d);
  assert(vec[0].v == 8);
  assert(d.v == 0);

  ConstMoveAssign m, m1, m2;
  const ConstMoveAssign cm;
  m1 = std::move(m);
  m2 = std::move(cm);
  assert(m1.mark == 1);
  assert(m2.mark == 10);
  return 0;
}
