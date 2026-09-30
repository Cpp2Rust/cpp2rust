// The unsafe model cannot express v[push_and_index(v)] without two borrows.
// no-compile: unsafe
#include <array>
#include <cassert>
#include <vector>

struct Point {
  int x;
  int y;
  int sum() const { return x + y; }
};

struct Holder {
  std::vector<int> values;
  std::vector<Point> points;
};

// Grows the vector while an index into it is being computed.
int push_and_index(std::vector<int> &v) {
  v.push_back(42);
  return static_cast<int>(v.size()) - 1;
}

int sum_ref(const std::vector<int> &v) {
  int s = 0;
  for (std::size_t i = 0; i < v.size(); ++i) {
    s += v[i];
  }
  return s;
}

int main() {
  std::vector<int> v = {1, 2, 3};
  v[0] = 10;
  assert(v[0] == 10);
  v[1] += 5;
  v[2]++;
  assert(v[1] == 7 && v[2] == 4);
  v[0] = v[1];
  assert(v[0] == 7);
  v[1] = 0;
  assert(v[v[1]] == 7);

  int i = 0;
  v[i++] = 3;
  assert(i == 1 && v[0] == 3);

  assert(v[push_and_index(v)] == 42);
  v[push_and_index(v)] = 5;
  assert(v.size() == 5 && v[4] == 5);

  int *p = &v[2];
  *p = 9;
  assert(v[2] == 9);
  assert(sum_ref(v) == 3 + 0 + 9 + 42 + 5);

  Holder h;
  h.values.resize(2);
  h.values[1] = 6;
  h.points.push_back({1, 2});
  h.points[0].y = 5;
  assert(h.values[1] == 6);
  assert(h.points[0].sum() == 6);

  Holder *hp = &h;
  hp->values[0] = hp->values[1] + 1;
  hp->points[0].x = hp->values[0];
  assert(h.values[0] == 7 && h.points[0].x == 7);

  Point q = h.points[0];
  q.x = 0;
  assert(h.points[0].x == 7);

  std::vector<std::vector<int>> grid;
  grid.push_back(std::vector<int>(3, 0));
  grid.push_back(std::vector<int>(3, 0));
  grid[1][2] = 8;
  assert(grid[1][2] == 8 && grid[0][2] == 0);

  std::array<int, 3> a = {4, 5, 6};
  a[1] = a[0] + a[2];
  assert(a[1] == 10);
  return 0;
}
