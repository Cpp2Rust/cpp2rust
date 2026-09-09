#include <cassert>

static int assigns = 0;

struct Partial {
  int v;
  int keep;
  Partial(int v, int keep) : v(v), keep(keep) {}
  Partial(const Partial &o) : v(o.v), keep(o.keep) {}
  Partial &operator=(const Partial &o) {
    if (this == &o) {
      return *this;
    }
    v = o.v;
    ++assigns;
    return *this;
  }
};

struct NonConstAssign {
  int mark;
  NonConstAssign() : mark(0) {}
  NonConstAssign &operator=(NonConstAssign &o) {
    mark = o.mark + 1;
    return *this;
  }
  NonConstAssign &operator=(const NonConstAssign &o) {
    mark = o.mark + 10;
    return *this;
  }
};

struct RefQualified {
  int mark;
  RefQualified() : mark(0) {}
  RefQualified &operator=(const RefQualified &o) & {
    mark = o.mark + 1;
    return *this;
  }
};

struct Holder {
  Partial p;
  Partial arr[2];
};

int main() {
  Partial a(1, 100), b(2, 200), c(3, 300);
  a = b;
  assert(a.v == 2 && a.keep == 100);
  assert(assigns == 1);

  c = a = b;
  assert(c.v == 2 && c.keep == 300);
  assert(assigns == 3);

  a = a;
  assert(assigns == 3);

  a = Partial(9, 900);
  assert(a.v == 9 && a.keep == 100);
  assert(assigns == 4);

  Partial &ra = a;
  ra = c;
  assert(a.v == 2);
  Partial *pa = &a;
  *pa = b;
  assert(a.v == 2);
  assert(assigns == 6);

  Holder h{Partial(4, 40), {Partial(5, 50), Partial(6, 60)}};
  h.p = b;
  h.arr[1] = c;
  assert(h.p.v == 2 && h.p.keep == 40);
  assert(h.arr[1].v == 2 && h.arr[1].keep == 60);
  assert(assigns == 8);

  NonConstAssign n, n1, n2;
  const NonConstAssign cn;
  n1 = n;
  n2 = cn;
  assert(n1.mark == 1);
  assert(n2.mark == 10);

  RefQualified r, r1;
  r1 = r;
  assert(r1.mark == 1);
  return 0;
}
