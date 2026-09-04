// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Tests arrow compound assignment, arrow read+write, and passing *ptr by value.
#include <cassert>
#include <memory>

struct Point {
  int x;
  int y;
};

int sum(Point p) { return p.x + p.y; }

int main() {
  auto p = std::make_unique<Point>(Point{3, 4});
  p->x += 10;
  p->y = p->x + p->y;
  int s = sum(*p);
  assert(s == 30);
  return 0;
}
