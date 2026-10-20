#include <algorithm>
#include <cassert>
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

bool operator<(const Tracked &a, const Tracked &b) { return a.v < b.v; }

bool greater(const Tracked &a, const Tracked &b) { return a.v > b.v; }

int main() {
  Tracked arr[5] = {Tracked(3), Tracked(1), Tracked(4), Tracked(1), Tracked(5)};
  std::sort(arr, arr + 5);
  assert(arr[0].v == 1 && arr[1].v == 1 && arr[2].v == 3 && arr[3].v == 4 &&
         arr[4].v == 5);
  assert(copies == 0);

  std::vector<Tracked> vec;
  vec.reserve(4);
  vec.emplace_back(2);
  vec.emplace_back(7);
  vec.emplace_back(1);
  vec.emplace_back(8);
  std::sort(vec.begin(), vec.end());
  assert(vec[0].v == 1 && vec[1].v == 2 && vec[2].v == 7 && vec[3].v == 8);
  assert(copies == 0);

  std::stable_sort(vec.begin(), vec.end(), greater);
  assert(vec[0].v == 8 && vec[3].v == 1);
  assert(copies == 0);

  std::stable_sort(
      vec.begin(), vec.end(),
      [](const Tracked &a, const Tracked &b) { return a.v < b.v; });
  assert(vec[0].v == 1 && vec[3].v == 8);
  assert(copies == 0);

  std::stable_sort(vec.begin() + 1, vec.end(), greater);
  assert(vec[0].v == 1 && vec[1].v == 8 && vec[3].v == 2);
  assert(copies == 0);
  return copies;
}
