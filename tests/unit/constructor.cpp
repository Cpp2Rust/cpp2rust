#include <cassert>

static int total = 0;

struct S {
  int v;
  int const_method() const { return v * 2; }
  void mut_method() { v += 1; }

  S(int init) : v(init) {
    mut_method();
    total += const_method();
  }

  ~S() {
    mut_method();
    total += this->const_method();
  }
};

struct Point {
  int x;
  int y;
  Point(int x, int y) : x(x), y(y) {}
  Point(int v) : Point(v, v + 1) { y *= 10; }
  Point() : Point(4) { x += 100; }
};

int main() {
  {
    S s(3);
    assert(s.v == 4);
    assert(total == 8);
  }
  assert(total == 18);
  Point p;
  assert(p.x == 104);
  assert(p.y == 50);
  Point q(7);
  assert(q.x == 7);
  assert(q.y == 80);
  return 0;
}
