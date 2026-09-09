#include <cassert>
#include <utility>
#include <vector>

static int copies = 0;

struct Counted {
  int v;
  Counted(int v) : v(v) {}
  Counted(const Counted &o) : v(o.v) { ++copies; }
};

struct NonConst {
  int mark;
  NonConst() : mark(0) {}
  NonConst(NonConst &o) : mark(o.mark + 1) {}
  NonConst(const NonConst &o) : mark(o.mark + 10) {}
};

struct Holder {
  Counted c;
  Counted arr[2];
};

static int by_value(Counted c) { return c.v; }

static Counted make(int v) {
  Counted c(v);
  return std::move(c);
}

int main() {
  Counted a(1);
  Counted b(a);
  Counted c = a;
  Counted d{a};
  assert(copies == 3);
  assert(b.v == 1 && c.v == 1 && d.v == 1);

  assert(by_value(a) == 1);
  assert(copies == 4);

  Counted e = make(5);
  assert(e.v == 5);
  assert(copies == 5);

  Counted f = Counted(6);
  assert(f.v == 6);
  assert(copies == 5);

  const Counted g(7);
  Counted h = g;
  assert(h.v == 7);
  assert(copies == 6);

  Holder hold{Counted(8), {Counted(9), Counted(10)}};
  Holder hold2 = hold;
  assert(hold2.c.v == 8 && hold2.arr[0].v == 9 && hold2.arr[1].v == 10);
  assert(copies == 9);

  std::vector<Counted> vec;
  vec.push_back(a);
  assert(vec[0].v == 1);
  assert(copies == 10);

  NonConst n;
  NonConst n1(n);
  const NonConst cn;
  NonConst n2(cn);
  assert(n1.mark == 1);
  assert(n2.mark == 10);
  return 0;
}
