// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <string>
#include <utility>
#include <vector>

// N4950 [sequence.reqmts]/104: a.push_back(rv) appends a copy of rv.
namespace p104_push_back_rvalue {
struct Point {
  int x;
  int y;
};

void test() {
  std::vector<int> ints;
  int value = 5;
  ints.push_back(std::move(value));
  assert(ints[0] == 5);
  assert(value == 5);

  int target = 7;
  std::vector<int *> pointers;
  int *pointer = &target;
  pointers.push_back(std::move(pointer));
  assert(*pointers[0] == 7);
  assert(pointer == &target);

  std::vector<Point> points;
  Point point{1, 2};
  points.push_back(std::move(point));
  assert(points[0].x == 1 && points[0].y == 2);
  assert(point.x == 1 && point.y == 2);

  std::vector<std::string> strings;
  std::string text = "text";
  strings.push_back(std::move(text));
  assert(strings[0] == "text");
}
} // namespace p104_push_back_rvalue

int main() {
  p104_push_back_rvalue::test();
  return 0;
}
