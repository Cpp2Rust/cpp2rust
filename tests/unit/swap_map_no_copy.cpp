#include <algorithm>
#include <cassert>
#include <map>
#include <memory>
#include <utility>
#include <vector>

static int copies = 0;

struct Tracked {
  int v;
  Tracked() : v(0) {}
  Tracked(int v) : v(v) {}
  Tracked(const Tracked &o) : v(o.v) { ++copies; }
  Tracked(Tracked &&o) noexcept : v(o.v) {}
  Tracked &operator=(const Tracked &o) {
    v = o.v;
    ++copies;
    return *this;
  }
  Tracked &operator=(Tracked &&o) noexcept {
    v = o.v;
    return *this;
  }
};

struct Pair {
  Tracked x;
  Tracked y;
};

struct Arrays {
  Tracked a[2];
  Tracked b[2];
};

static int copy_only_copies = 0;
static int copy_only_dtors = 0;

// The copy constructor suppresses the move constructor, so std::swap copies.
struct CopyOnly {
  int v;
  CopyOnly(int v) : v(v) {}
  CopyOnly(const CopyOnly &o) : v(o.v) { ++copy_only_copies; }
  ~CopyOnly() { ++copy_only_dtors; }
  CopyOnly &operator=(const CopyOnly &o) {
    v = o.v;
    ++copy_only_copies;
    return *this;
  }
};

struct CopyOnlyPair {
  CopyOnly x;
  CopyOnly y;
};

static int moves = 0;
static int move_assigns = 0;
static int moved_dtors = 0;

// std::swap moves into a temporary, move assigns twice, and destroys the
// temporary.
struct Moved {
  int v;
  Moved(int v) : v(v) {}
  Moved(const Moved &o) : v(o.v) { ++copies; }
  Moved(Moved &&o) noexcept : v(o.v) {
    o.v = -1;
    ++moves;
  }
  Moved &operator=(const Moved &o) {
    v = o.v;
    ++copies;
    return *this;
  }
  Moved &operator=(Moved &&o) noexcept {
    v = o.v;
    o.v = -1;
    ++move_assigns;
    return *this;
  }
  ~Moved() { ++moved_dtors; }
};

struct MovedPair {
  Moved x;
  Moved y;
};

static bool swapped_once() {
  bool ok = moves == 1 && move_assigns == 2 && moved_dtors == 1;
  moves = move_assigns = moved_dtors = 0;
  return ok;
}

struct Owner {
  std::unique_ptr<int> p;
};

int main() {
  Tracked a(1);
  Tracked b(2);
  std::swap(a, b);
  assert(a.v == 2 && b.v == 1);
  assert(copies == 0);

  std::map<int, Tracked> m1;
  m1[1] = Tracked(10);
  m1[2] = Tracked(20);
  std::map<int, Tracked> m2;
  m2[3] = Tracked(30);
  assert(copies == 0);
  std::swap(m1[1], m1[2]);
  assert(m1[1].v == 20 && m1[2].v == 10);
  assert(copies == 0);
  std::swap(m1, m2);
  assert(m1.size() == 1 && m1[3].v == 30);
  assert(m2.size() == 2 && m2[1].v == 20 && m2[2].v == 10);
  assert(copies == 0);

  Tracked arr[3] = {Tracked(1), Tracked(2), Tracked(3)};
  std::swap(arr[0], arr[2]);
  assert(arr[0].v == 3 && arr[2].v == 1);
  std::swap(arr[1], arr[1]);
  assert(arr[1].v == 2);
  assert(copies == 0);

  std::vector<Tracked> vec;
  vec.emplace_back(4);
  vec.emplace_back(5);
  std::swap(vec[0], vec[1]);
  assert(vec[0].v == 5 && vec[1].v == 4);
  assert(copies == 0);

  Pair pairs[2] = {{Tracked(6), Tracked(7)}, {Tracked(8), Tracked(9)}};
  Pair p{Tracked(10), Tracked(11)};
  std::swap(pairs[0].x, p.y);
  assert(pairs[0].x.v == 11 && p.y.v == 6);
  std::swap(pairs[0], pairs[1]);
  assert(pairs[0].x.v == 8 && pairs[1].x.v == 11);
  assert(copies == 0);

  std::swap(p.x, p.y);
  assert(p.x.v == 6 && p.y.v == 10);
  std::swap(pairs[0].x, pairs[1].y);
  assert(pairs[0].x.v == 7 && pairs[1].y.v == 8);
  assert(copies == 0);

  Arrays arrays;
  arrays.a[0].v = 1;
  arrays.b[1].v = 2;
  std::swap(arrays.a[0], arrays.b[1]);
  assert(arrays.a[0].v == 2 && arrays.b[1].v == 1);
  std::swap(arrays.a[0], arrays.a[1]);
  assert(arrays.a[0].v == 0 && arrays.a[1].v == 2);
  assert(copies == 0);

  std::vector<std::vector<Tracked>> grid(2);
  grid[0].emplace_back(1);
  grid[0].emplace_back(2);
  grid[1].emplace_back(3);
  std::swap(grid[0][1], grid[1][0]);
  assert(grid[0][1].v == 3 && grid[1][0].v == 2);
  std::swap(grid[0], grid[1]);
  assert(grid[0].size() == 1 && grid[0][0].v == 2);
  assert(copies == 0);

  std::map<int, Tracked> maps[2];
  maps[0][1] = Tracked(1);
  maps[1][2] = Tracked(2);
  std::swap(maps[0], maps[1]);
  assert(maps[0].size() == 1 && maps[0][2].v == 2);
  assert(maps[1].size() == 1 && maps[1][1].v == 1);
  assert(copies == 0);

  CopyOnly c1(1);
  CopyOnly c2(2);
  std::swap(c1, c2);
  assert(c1.v == 2 && c2.v == 1);
  assert(copy_only_copies == 3 && copy_only_dtors == 1);
  CopyOnlyPair cp{CopyOnly(3), CopyOnly(4)};
  std::swap(cp.x, cp.y);
  assert(cp.x.v == 4 && cp.y.v == 3);
  assert(copy_only_copies == 6 && copy_only_dtors == 2);
  CopyOnly carr[2] = {CopyOnly(5), CopyOnly(6)};
  std::swap(carr[0], carr[1]);
  assert(carr[0].v == 6 && carr[1].v == 5);
  assert(copy_only_copies == 9 && copy_only_dtors == 3);

  Moved mv1(1);
  Moved mv2(2);
  std::swap(mv1, mv2);
  assert(mv1.v == 2 && mv2.v == 1);
  assert(swapped_once());
  MovedPair mp{Moved(3), Moved(4)};
  std::swap(mp.x, mp.y);
  assert(mp.x.v == 4 && mp.y.v == 3);
  assert(swapped_once());
  Moved marr[2] = {Moved(5), Moved(6)};
  std::swap(marr[0], marr[1]);
  assert(marr[0].v == 6 && marr[1].v == 5);
  assert(swapped_once());
  std::swap(mv1, mv1);
  assert(mv1.v == 2);
  assert(swapped_once());
  assert(copies == 0);

  Owner o1{std::make_unique<int>(1)};
  Owner o2{std::make_unique<int>(2)};
  std::swap(o1, o2);
  assert(*o1.p == 2 && *o2.p == 1);
  return copies;
}
