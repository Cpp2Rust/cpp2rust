// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <any>
#include <cassert>

struct Point {
  int x;
  int y;
  Point(int x, int y) : x(x), y(y) {}
};

int main() {
  std::any a = std::make_any<Point>(3, 4);
  assert(a.has_value());
  Point *p = std::any_cast<Point>(&a);
  assert(p != nullptr);
  assert(p->x == 3);
  assert(p->y == 4);
  assert(std::any_cast<int>(&a) == nullptr);
  Point &q = a.emplace<Point>(5, 6);
  assert(q.x == 5);
  assert(std::any_cast<Point>(&a)->y == 6);
  std::any b = std::make_any<int>(7);
  assert(*std::any_cast<int>(&b) == 7);
  b.reset();
  assert(!b.has_value());
  assert(std::any_cast<int>(&b) == nullptr);
  return 0;
}
