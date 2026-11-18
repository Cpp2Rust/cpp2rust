// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <utility>
#include <vector>

// N4950 [forward]/9: std::move(t) returns
// static_cast<remove_reference_t<T>&&>(t), so moving a scalar reads its value.
namespace p9_move_scalar {
void test() {
  int source = 1;
  int target = std::move(source);
  assert(target == 1);
  assert(source == 1);
}
} // namespace p9_move_scalar

// N4950 [forward]/9: moving a trivially copyable object copies its value.
namespace p9_move_trivially_copyable {
struct Point {
  int x;
  int y;
};

void test() {
  Point source{1, 2};
  Point target = std::move(source);
  assert(target.x == 1 && target.y == 2);
  assert(source.x == 1 && source.y == 2);

  Point assigned{0, 0};
  Point other{3, 4};
  assigned = std::move(other);
  assert(assigned.x == 3 && assigned.y == 4);
  assert(other.x == 3 && other.y == 4);

  std::vector<Point> points;
  Point element{5, 6};
  points.push_back(std::move(element));
  assert(points[0].x == 5 && points[0].y == 6);
  assert(element.x == 5 && element.y == 6);
}
} // namespace p9_move_trivially_copyable

// N4950 [forward]/10: std::move(a) binds to the constructor A(A&&), which
// moves the value from a.
namespace p10_move_constructor {
struct A {
  int value;
  A(int value) : value(value) {}
  A(A &&other) : value(other.value) { other.value = 0; }
};

void test() {
  A source(1);
  A target = std::move(source);
  assert(target.value == 1);
  assert(source.value == 0);

  std::vector<A> values;
  A element(2);
  values.push_back(std::move(element));
  assert(values[0].value == 2);
  assert(element.value == 0);
}
} // namespace p10_move_constructor

int main() {
  p9_move_scalar::test();
  p9_move_trivially_copyable::test();
  p10_move_constructor::test();
  return 0;
}
